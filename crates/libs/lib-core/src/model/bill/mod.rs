use crate::model::ModelManager;
use lib_utils::b58::b58_encode;
use rand::RngCore as _;
use serde::{Deserialize, Serialize};

mod error;

pub use error::{Error, Result};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub bill_id: String,
    pub remark: Option<String>,
    pub seller_id: Option<String>,
}

/// Fields required for creating new bill
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillForCreate {
    remark: Option<String>,
    seller_id: Option<String>,
}

pub struct BillBmc;

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
        mm: &ModelManager,
        bill_c: BillForCreate,
    ) -> Result<String> {
        let BillForCreate { remark, seller_id } = bill_c;

        // Start the transaction
        let mm = mm.new_with_txn();
        mm.dbx().begin_txn().await?;

        // region:    --- Insert bill
        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "insert into bill (bill_id, remark, seller_serial_id)
            values ($1, $2,
            (select serial_id from seller s where s.seller_id = $3 limit 1))
            on conflict (bill_id) do nothing
            returning bill_id;",
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

    pub async fn get_by_bill_id(
        mm: &ModelManager,
        bill_id: &str,
    ) -> Result<Bill> {
        let sqlx_query = sqlx::query_as::<_, Bill>(
            "select b.bill_id, b.remark, s.seller_id from bill b
            inner join seller s on b.seller_serial_id = s.serial_id
            where b.bill_id = $1 limit 1;",
        )
        .bind(bill_id);

        let user = mm.dbx().fetch_optional(sqlx_query).await?.ok_or(
            Error::BillNotFound {
                bill_id: bill_id.to_string(),
            },
        )?;

        Ok(user)
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    pub type Result<T> = std::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>; // For tests.

    use super::*;
    use crate::{_dev_utils, model::seller::SellerBmc};
    use serde_json::json;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let fx_seller_name = "test_create_ok-seller_name-01";
        let fx_remark = Some(String::from("test_create_ok-bill_remark-01"));

        // SellerForCreate::
        let sellers = SellerBmc::create(
            &mm,
            serde_json::from_value(json!([
                { "name": fx_seller_name  }
            ]))
            .unwrap(),
        )
        .await?;

        // -- Exec
        let bill_id = BillBmc::create(
            &mm,
            BillForCreate {
                remark: fx_remark.clone(),
                seller_id: Some(sellers[0].seller_id.clone()),
            },
        )
        .await?;

        // -- Check
        let bill: Bill = BillBmc::get_by_bill_id(&mm, &bill_id).await?;
        assert_eq!(bill.remark, fx_remark);

        // // -- Clean
        // SellerBmc::delete(&ctx, &mm, &seller.seller_id).await?;

        Ok(())
    }
}

// endregion: --- Tests
