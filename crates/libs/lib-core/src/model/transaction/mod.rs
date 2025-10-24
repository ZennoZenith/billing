use std::{collections::HashSet, str::FromStr};

use crate::model::ModelManager;
use bigdecimal::BigDecimal;
use lib_utils::{b58::b58_encode, time::TimeRfc3339};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};

mod error;

pub use error::{Error, Result};

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
    strum_macros::EnumString,
)]
#[strum(ascii_case_insensitive)]
pub enum PaymentMethod {
    Card,
    Cash,
    Upi,
    // ...
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitCost {
    unit_type: String,
    unit: BigDecimal,
    cost_per_unit: BigDecimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionForCreate {
    name: String,
    transaction_time: Option<TimeRfc3339>,
    remark: Option<String>,
    tags: Option<Vec<String>>,
    payment_method: PaymentMethod,
    unit_cost: Option<UnitCost>,
    seller_cost: BigDecimal,
    bill_id: Option<String>,
}

/// Fields required for creating new transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    transaction_id: String,
    name: String,
    transaction_time: TimeRfc3339,
    remark: Option<String>,
    tags: Vec<String>,
    payment_method: PaymentMethod,
    unit_cost: Option<UnitCost>,
    seller_cost: BigDecimal,
    bill_id: Option<String>,
}

pub struct TransactionBmc;

impl TransactionBmc {
    fn generate_transaction_id() -> String {
        let mut key = [0u8; 64]; // 512 bits = 64 bytes
        rand::rng().fill_bytes(&mut key);
        b58_encode(key)
            .chars()
            .take(20)
            .collect::<String>()
            .to_uppercase()
    }

    pub async fn create(
        mm: &ModelManager,
        transaction_c: Vec<TransactionForCreate>,
    ) -> Result<Vec<Transaction>> {
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
            .map(|v| (v.unit_type.trim(), v.cost_per_unit.clone()));

        // Start the transaction
        let mm = mm.new_with_txn();
        mm.dbx().begin_txn().await?;

        // region:    --- Insert tags
        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "select name from unnest($1::text[]) as t(name)
            where t.name not in (select name from tag);",
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
            "insert into tag (name, ctime, mtime)
            select * from unnest(
                $1::text[],
                $2::timestamptz[],
                $3::timestamptz[]
            ) as t(name, ctime, mtime)
            on conflict (name) do nothing;",
        )
        .bind(new_tags)
        .bind(ctimes)
        .bind(mtimes);

        mm.dbx().execute(sqlx_query).await?;

        // endregion: --- Insert tags

        // region:    --- Insert unit_type

        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "select name from unnest($1::text[]) as t(name)
            where t.name not in (select name from unit_type);",
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
            "insert into unit_type (name, ctime, mtime)
            select * from unnest(
                $1::text[],
                $2::timestamptz[],
                $3::timestamptz[]
            ) as t(name, ctime, mtime)
            on conflict (name) do nothing;",
        )
        .bind(new_unit_types)
        .bind(ctimes)
        .bind(mtimes);

        mm.dbx().execute(sqlx_query).await?;
        // endregion: --- Insert unit_type

        // region:    --- Insert unit_cost

        let (unit_types, costs_per_unit): (Vec<&str>, Vec<BigDecimal>) =
            per_unit_costs.unzip();

        let sqlx_query = sqlx::query_as::<_, (String, BigDecimal)>(
            "select unit_type, cost_per_unit from unnest(
                $1::text[],
                $2::numeric(12, 2)[]
        	 ) as t(unit_type, cost_per_unit)
            where 
            (t.unit_type, t.cost_per_unit)
            not in (
                select unit_type, cost_per_unit from unit_cost uc
                inner join unit_type ut
                on uc.unit_type_serial_id = ut.serial_id
            );",
        )
        .bind(unit_types)
        .bind(costs_per_unit);

        let (new_unit_types, new_unit_types_costs): (
            Vec<String>,
            Vec<BigDecimal>,
        ) = mm.dbx().fetch_all(sqlx_query).await?.into_iter().unzip();

        let sqlx_query = sqlx::query(
            "insert into unit_cost (unit_type_serial_id, cost_per_unit) 
                select ut.serial_id, t.cost_per_unit from
                unnest(
                    $1::text[],
                    $2::float8[]
            	) as t(unit_type, cost_per_unit) inner join unit_type ut on t.unit_type = ut.name;",
        )
        .bind(new_unit_types)
        .bind(new_unit_types_costs);

        mm.dbx().execute(sqlx_query).await?;
        // endregion: --- Insert unit_cost

        let mut completed_transaction_ids: Vec<String> =
            Vec::with_capacity(transaction_c.len());

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

            let sqlx_query = sqlx::query_as::<_, (String, i64)>(
                "insert into transaction (transaction_id, name, remark, transaction_time, payment_method_serial_id, seller_cost, bill_serial_id) values
                ($1, $2, $3, $4, (select serial_id from payment_method p where p.name = $5 limit 1), $6,
                (select serial_id from bill b where b.bill_id = $7 limit 1) )
                returning transaction_id, serial_id;",
            )
            .bind(TransactionBmc::generate_transaction_id())
            .bind(name)
            .bind(remark)
            .bind(transaction_time.map(|v| v.inner()).unwrap_or(now))
            .bind(payment_method.to_string())
            .bind(seller_cost)
            .bind(bill_id);

            let (transaction_id, transaction_serial_id) =
                mm.dbx().fetch_one(sqlx_query).await?;

            println!("transaction id: {}", transaction_id);
            completed_transaction_ids.push(transaction_id);

            if let Some(tags) = tags {
                for tag in tags {
                    let sqlx_query = sqlx::query(
                    "insert into transaction_tag (transaction_serial_id, tag_serial_id) values
                    ($1, (select serial_id from tag t where t.name = $2 limit 1))",
                )
                .bind(transaction_serial_id)
                .bind(tag.trim().to_lowercase());

                    mm.dbx().execute(sqlx_query).await?;
                }
            }

            if let Some(unit) = unit_cost {
                let sqlx_query = sqlx::query(
                    "insert into transaction_unit (transaction_serial_id, unit,unit_cost_serial_id) values
                    ($1, $2,
                    (select uc.serial_id from unit_cost uc where
                    (uc.unit_type_serial_id, uc.cost_per_unit) <> ((select ut.serial_id from unit_type ut where ut.name = $3), $4) limit 1))",
                )
                .bind(transaction_serial_id)
                .bind(unit.unit)
                .bind(unit.unit_type.trim())
                .bind(unit.cost_per_unit);

                mm.dbx().execute(sqlx_query).await?;
            }
        }

        // Commit the transaction
        mm.dbx().commit_txn().await?;

        let transactions =
            Self::get_by_transaction_ids(&mm, completed_transaction_ids)
                .await?;

        Ok(transactions)
    }

    pub async fn get_by_transaction_ids(
        mm: &ModelManager,
        transaction_ids: Vec<String>,
    ) -> Result<Vec<Transaction>> {
        #[derive(sqlx::FromRow)]
        struct TransactionTable {
            serial_id: i64,
            transaction_id: String,
            name: String,
            remark: Option<String>,
            transaction_time: TimeRfc3339,
            payment_method_name: String,
            seller_cost: BigDecimal,
            bill_id: Option<String>,
        }

        #[derive(sqlx::FromRow)]
        struct TransactionTagsTable {
            transaction_serial_id: i64,
            tag_name: String,
        }

        #[derive(Debug, sqlx::FromRow)]
        struct TransactionUnitTable {
            transaction_serial_id: i64,
            unit: BigDecimal,
            cost_per_unit: BigDecimal,
            unit_type_name: String,
        }

        let sqlx_query = sqlx::query_as::<_, TransactionTable>(
            "select tr.serial_id, tr.transaction_id, tr.name,
                tr.remark, tr.transaction_time,
                pm.name as payment_method_name,
                tr.seller_cost,
                bl.bill_id from transaction tr
            inner join payment_method pm
                on pm.serial_id = tr.payment_method_serial_id
            left join bill bl
                on bl.serial_id = tr.bill_serial_id
            where transaction_id in (select unnest($1::text[]));
           ",
        )
        .bind(&transaction_ids);

        let transactions = mm.dbx().fetch_all(sqlx_query).await?;
        let transaction_serial_ids = transactions
            .iter()
            .map(|v| v.serial_id)
            .collect::<Vec<i64>>();

        let sqlx_query = sqlx::query_as::<_, TransactionTagsTable>(
            "select tr.transaction_serial_id, tg.name as tag_name from transaction_tag tr
            inner join tag tg
            on tr.tag_serial_id = tg.serial_id
            where tr.transaction_serial_id in (select unnest($1::bigint[]));
           ",
        )
        .bind(&transaction_serial_ids);

        let tags = mm.dbx().fetch_all(sqlx_query).await?;

        let sqlx_query = sqlx::query_as::<_, TransactionUnitTable>(
            "select tu.transaction_serial_id, tu.unit,
                uc.cost_per_unit, ut.name as unit_type_name
            from transaction_unit tu
            inner join unit_cost uc
                on tu.unit_cost_serial_id = uc.serial_id
            inner join unit_type ut
                on uc.unit_type_serial_id = ut.serial_id
            where tu.transaction_serial_id in (select unnest($1::bigint[]));
           ",
        )
        .bind(&transaction_serial_ids);

        let transactions_unit = mm.dbx().fetch_all(sqlx_query).await?;

        println!("{:#?}", transactions_unit);

        Ok(transactions
            .into_iter()
            .map(|v| Transaction {
                transaction_id: v.transaction_id,
                name: v.name,
                transaction_time: v.transaction_time,
                remark: v.remark,
                tags: tags
                    .iter()
                    .filter(|t| t.transaction_serial_id == v.serial_id)
                    .map(|t| t.tag_name.clone())
                    .collect::<Vec<String>>(),
                payment_method: PaymentMethod::from_str(&v.payment_method_name)
                    .unwrap_or_default(),
                unit_cost: transactions_unit
                    .iter()
                    .find(|t| t.transaction_serial_id == v.serial_id)
                    .map(|t| UnitCost {
                        unit_type: t.unit_type_name.clone(),
                        unit: t.unit.clone(),
                        cost_per_unit: t.cost_per_unit.clone(),
                    }),
                seller_cost: v.seller_cost,
                bill_id: v.bill_id,
            })
            .collect())
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    pub type Result<T> = std::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>; // For tests.

    use super::*;
    use crate::{
        _dev_utils,
        model::{bill::BillBmc, seller::SellerBmc},
    };
    use serde_json::json;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;

        let sellers = SellerBmc::create(
            &mm,
            serde_json::from_value(json!([
                { "name": "test_create_ok-seller_name-01" }
            ]))
            .unwrap(),
        )
        .await?;

        let bill_id = BillBmc::create(
            &mm,
            serde_json::from_value(json!({
                "remark": "test_create_ok-bill_remark-01",
                "sellerId": sellers[0].seller_id
            }))
            .unwrap(),
        )
        .await?;

        let transactions = TransactionBmc::create(
            &mm,
            serde_json::from_value(json!([{
              "name": "tr 1",
              "transactionTime": null,
              "remark": null,
              "tags": ["a", "b "],
              "paymentMethod": "Cash",
              "unitCost": {
                "unitType": "kg",
                "unit": 1.4,
                "costPerUnit": 58.0
              },
              "sellerCost": "10.4321",
              "billId": null
            }, {
              "name": "tr 2",
              "transactionTime": "2020-09-08T13:10:08.511Z",
              "remark": "Some remark",
              "tags": ["d", "c"],
              "paymentMethod": "Upi",
              "unitCost": {
                "unitType": "bag",
                "unit": 5,
                "costPerUnit": 340
              },
              "sellerCost": 1700,
              "billId": bill_id
            }]))
            .unwrap(),
        )
        .await?;

        println!("{:?}", transactions);

        let transaction_ids = transactions
            .iter()
            .map(|v| v.transaction_id.clone())
            .collect();

        let transactions =
            TransactionBmc::get_by_transaction_ids(&mm, transaction_ids)
                .await?;

        println!("{}", serde_json::to_string_pretty(&transactions).unwrap());

        // // -- Clean

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;

        let sellers = SellerBmc::create(
            &mm,
            serde_json::from_value(json!([
                { "name": "test_create_ok-seller_name-01" }
            ]))
            .unwrap(),
        )
        .await?;

        let bill_id = BillBmc::create(
            &mm,
            serde_json::from_value(json!({
                "remark": "test_create_ok-bill_remark-01",
                "sellerId": sellers[0].seller_id
            }))
            .unwrap(),
        )
        .await?;

        // -- Exec
        let transactions = TransactionBmc::create(
            &mm,
            serde_json::from_value(json!([{
                "name": "tr 1",
                "transactionTime": null,
                "remark": null,
                "tags": ["a", "b "],
                "paymentMethod": "Cash",
                "unitCost": {
                    "unitType": "kg",
                    "unit": 1.4,
                    "costPerUnit": 58.0
                },
                "sellerCost": "10.4321",
                "billId": null
            }, {
                "name": "tr 2",
                "transactionTime": "2020-09-08T13:10:08.511Z",
                "remark": "Some remark",
                "tags": ["d", "c"],
                "paymentMethod": "Upi",
                "unitCost": {
                    "unitType": "bag",
                    "unit": 5,
                    "costPerUnit": 340
                },
                "sellerCost": 1700,
                "billId": bill_id
            }, {
                "name": "tr 3",
                "transactionTime": "2020-09-08T13:10:08.511Z",
                "remark": null,
                "tags": ["a", "c"],
                "paymentMethod": "Card",
                "unitCost": null,
                "sellerCost": 1700,
                "billId": bill_id
            }]))
            .unwrap(),
        )
        .await?;

        println!("{:?}", transactions);

        let transaction_ids = transactions
            .iter()
            .map(|v| v.transaction_id.clone())
            .collect();

        let transactions =
            TransactionBmc::get_by_transaction_ids(&mm, transaction_ids)
                .await?;

        println!("{}", serde_json::to_string_pretty(&transactions).unwrap());

        Ok(())
    }
}

// endregion: --- Tests
