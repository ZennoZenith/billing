use crate::model::{ModelManager, base::DbBmc};
use lib_utils::b58::b58_encode;
use rand::RngCore as _;
use serde::Deserialize;

mod error;

pub use error::{Error, Result};

/// Fields required for creating new bill
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillForCreate {
    remark: Option<String>,
    seller_id: Option<String>,
}

pub struct BillBmc;

impl DbBmc for BillBmc {
    const TABLE: &'static str = "bill";
}

impl BillBmc {
    fn generate_bill_id() -> String {
        let mut key = [0u8; 64]; // 512 bits = 64 bytes
        rand::rng().fill_bytes(&mut key);
        b58_encode(key)
            .chars()
            .take(10)
            .collect::<String>()
            .to_uppercase()
    }

    pub async fn create(
        // ctx: &Ctx,
        mm: &ModelManager,
        bill_c: BillForCreate,
    ) -> Result<String> {
        let BillForCreate { remark, seller_id } = bill_c;

        // Start the transaction
        let mm = mm.new_with_txn();
        mm.dbx().begin_txn().await?;

        // region:    --- Insert bill
        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "INSERT INTO bill (bill_id, remark, seller_serial_id)
            VALUES ($1, $2,
            (SELECT serial_id FROM seller s WHERE s.seller_id = $3 LIMIT 1))
            ON CONFLICT (bill_id) DO NOTHING
            RETURNING bill_id;",
        )
        .bind(BillBmc::generate_bill_id())
        .bind(remark)
        .bind(seller_id);

        let (returning_bill_id,) = mm.dbx().fetch_one(sqlx_query).await?;

        // Commit the transaction
        mm.dbx().commit_txn().await?;

        // endregion: --- Insert bill

        Ok(returning_bill_id)
    }
}
