use std::collections::HashSet;

use crate::model::{ModelManager, base::DbBmc};
use lib_utils::time::TimeRfc3339;
use serde::{Deserialize, Serialize};

mod error;

pub use error::{Error, Result};

#[derive(Clone, Debug, Deserialize, Serialize, strum_macros::Display)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    #[strum(to_string = "cash")]
    Cash,
    #[strum(to_string = "upi")]
    Upi,
    #[strum(to_string = "card")]
    Card,
    // ...
    #[strum(to_string = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitCost {
    unit_type: String,
    unit: f64,
    cost_per_unit: f64,
}

/// Fields required for creating new transaction
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionForCreate {
    name: String,
    transaction_time: Option<TimeRfc3339>,
    remark: Option<String>,
    tags: Option<Vec<String>>,
    payment_method: PaymentMethod,
    unit_cost: Option<UnitCost>,
    seller_cost: f64,
    bill_id: Option<String>,
}

pub struct TransactionBmc;

impl DbBmc for TransactionBmc {
    const TABLE: &'static str = "transaction";
}

impl TransactionBmc {
    pub async fn create(
        // ctx: &Ctx,
        mm: &ModelManager,
        transaction_c: Vec<TransactionForCreate>,
    ) -> Result<()> {
        let now = TimeRfc3339::now_utc().inner();

        let tags = transaction_c
            .iter()
            .filter_map(|v| v.tags.as_ref())
            .flatten()
            .map(|v| v.trim().to_lowercase())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<String>>();

        let unit_types = transaction_c
            .iter()
            .filter_map(|v| v.unit_cost.as_ref())
            .map(|v| v.unit_type.trim())
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect::<Vec<&str>>();

        let per_unit_costs = transaction_c
            .iter()
            .filter_map(|v| v.unit_cost.as_ref())
            .map(|v| (v.unit_type.trim(), v.cost_per_unit));

        // Start the transaction
        let mm = mm.new_with_txn();
        mm.dbx().begin_txn().await?;

        // region:    --- Insert tags
        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "SELECT name FROM UNNEST($1::text[]) AS t(name)
            WHERE t.name NOT IN (SELECT name FROM tag);",
        )
        .bind(&tags);

        let new_tags: Vec<String> = mm
            .dbx()
            .fetch_all(sqlx_query)
            .await?
            .into_iter()
            .map(|v| v.0)
            .collect();

        let ctimes = vec![now; new_tags.len()];
        let mtimes = vec![now; new_tags.len()];

        let sqlx_query = sqlx::query(
            "INSERT INTO tag (name, ctime, mtime)
            SELECT * FROM UNNEST(
                $1::text[],
                $2::timestamptz[],
                $3::timestamptz[]
            ) as t(name, ctime, mtime)
            ON CONFLICT (name) DO NOTHING;",
        )
        .bind(new_tags)
        .bind(ctimes)
        .bind(mtimes);

        mm.dbx().execute(sqlx_query).await?;

        // endregion: --- Insert tags

        // region:    --- Insert unit_type

        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "SELECT name FROM UNNEST($1::text[]) AS t(name)
            WHERE t.name NOT IN (SELECT name FROM unit_type);",
        )
        .bind(&unit_types);

        let new_unit_types: Vec<String> = mm
            .dbx()
            .fetch_all(sqlx_query)
            .await?
            .into_iter()
            .map(|v| v.0)
            .collect();

        let ctimes = vec![now; new_unit_types.len()];
        let mtimes = vec![now; new_unit_types.len()];

        let sqlx_query = sqlx::query(
            "INSERT INTO unit_type (name, ctime, mtime)
            SELECT * FROM UNNEST(
                $1::text[],
                $2::timestamptz[],
                $3::timestamptz[]
            ) as t(name, ctime, mtime)
            ON CONFLICT (name) DO NOTHING;",
        )
        .bind(new_unit_types)
        .bind(ctimes)
        .bind(mtimes);

        mm.dbx().execute(sqlx_query).await?;
        // endregion: --- Insert unit_type

        // region:    --- Insert unit_cost

        let (unit_types, costs_per_unit): (Vec<&str>, Vec<f64>) =
            per_unit_costs.unzip();

        let sqlx_query = sqlx::query_as::<_, (String, f64)>(
            "SELECT unit_type, cost_per_unit FROM UNNEST(
                $1::text[],
                $2::float8[]
        	 ) AS t(unit_type, cost_per_unit)
            WHERE 
            (t.unit_type, t.cost_per_unit)
            NOT IN (
                SELECT unit_type, cost_per_unit FROM unit_cost uc
                INNER JOIN unit_type ut
                ON uc.unit_type_serial_id = ut.serial_id
            );",
        )
        .bind(unit_types)
        .bind(costs_per_unit);

        let (new_unit_types, new_unit_types_costs): (Vec<String>, Vec<f64>) =
            mm.dbx().fetch_all(sqlx_query).await?.into_iter().unzip();

        let sqlx_query = sqlx::query(
            "INSERT INTO unit_cost (unit_type_serial_id, cost_per_unit) 
                SELECT ut.serial_id, t.cost_per_unit FROM
                UNNEST(
                    $1::text[],
                    $2::float8[]
            	) AS t(unit_type, cost_per_unit) INNER JOIN unit_type ut ON t.unit_type = ut.name;",
        )
        .bind(new_unit_types)
        .bind(new_unit_types_costs);

        mm.dbx().execute(sqlx_query).await?;
        // endregion: --- Insert unit_cost

        for transaction in transaction_c {
            let TransactionForCreate {
                name,
                transaction_time,
                remark,
                tags,
                payment_method,
                unit_cost,
                seller_cost,
                bill_id,
            } = transaction;

            let sqlx_query = sqlx::query_as::<_, (i64,)>(
                "INSERT INTO transaction (name, remark, transaction_time, payment_method_serial_id, seller_cost, bill_serial_id) VALUES
                ($1, $2, $3, (SELECT serial_id FROM payment_method p WHERE p.name = $4 LIMIT 1), $5,
                (SELECT serial_id FROM bill b WHERE b.bill_id = $6 LIMIT 1) )
                returning serial_id;",
            )
            .bind(name)
            .bind(remark)
            .bind(transaction_time.map(|v| v.inner()).unwrap_or(now))
            .bind(payment_method.to_string())
            .bind(seller_cost)
            .bind(bill_id);

            let (transaction_id,) = mm.dbx().fetch_one(sqlx_query).await?;

            println!("transaction id: {}", transaction_id);

            if let Some(tags) = tags {
                for tag in tags {
                    let sqlx_query = sqlx::query(
                    "INSERT INTO transaction_tag (transaction_serial_id, tag_serial_id) VALUES
                    ($1, (SELECT serial_id FROM tag t WHERE t.name = $2 LIMIT 1))",
                )
                .bind(transaction_id)
                .bind(tag.trim().to_lowercase());

                    mm.dbx().execute(sqlx_query).await?;
                }
            }

            if let Some(unit) = unit_cost {
                let sqlx_query = sqlx::query(
                    "INSERT INTO transaction_unit (transaction_serial_id, unit,unit_cost_serial_id) VALUES
                    ($1, $2,
                    (SELECT uc.serial_id FROM unit_cost uc WHERE
                    (uc.unit_type_serial_id, uc.cost_per_unit) <> ((SELECT ut.serial_id FROM unit_type ut WHERE ut.name = $3), $4) LIMIT 1))",
                )
                .bind(transaction_id)
                .bind(unit.unit)
                .bind(unit.unit_type.trim())
                .bind(unit.cost_per_unit);

                mm.dbx().execute(sqlx_query).await?;
            }
        }

        // Commit the transaction
        mm.dbx().commit_txn().await?;

        let sqlx_query = sqlx::query_as::<_, (i64, String)>(
            "SELECT serial_id, name FROM unit_type;",
        )
        .bind(&tags);

        let all_entity: Vec<_> = mm.dbx().fetch_all(sqlx_query).await?;
        println!("{:?}", all_entity);

        Ok(())
    }
}
