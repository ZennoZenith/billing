use std::collections::HashSet;

use crate::model::{ModelManager, base::DbBmc};
use lib_utils::b58::b58_encode;
use rand::RngCore as _;
use serde::{Deserialize, Serialize};

mod error;

pub use error::{Error, Result};
use sqlx::prelude::FromRow;

/// Fields required for creating new bill
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SellerForCreate {
    name: String,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Seller {
    name: String,
    seller_id: String,
}

pub struct SellerBmc;

impl DbBmc for SellerBmc {
    const TABLE: &'static str = "seller";
}

impl SellerBmc {
    fn generate_seller_id() -> String {
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
        sellers_c: Vec<SellerForCreate>,
    ) -> Result<Vec<Seller>> {
        let sellers = sellers_c
            .iter()
            .map(|v| v.name.trim())
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect::<Vec<&str>>();

        // Start the transaction
        let mm = mm.new_with_txn();
        mm.dbx().begin_txn().await?;

        // region:    --- Insert Sellers
        let sqlx_query = sqlx::query_as::<_, (String,)>(
            "SELECT name FROM UNNEST($1::text[]) AS t(name)
            WHERE t.name NOT IN (SELECT name FROM seller);",
        )
        .bind(&sellers);

        let new_sellers: Vec<String> = mm
            .dbx()
            .fetch_all(sqlx_query)
            .await?
            .into_iter()
            .map(|v| v.0)
            .collect();

        let mut seller_ids = Vec::with_capacity(new_sellers.len());
        (0..new_sellers.len())
            .for_each(|_| seller_ids.push(SellerBmc::generate_seller_id()));

        let sqlx_query = sqlx::query_as::<_, Seller>(
            "INSERT INTO seller (name, seller_id)
            SELECT * FROM UNNEST(
                $1::text[],
                $2::text[]
            ) as t(name, seller_id)
            ON CONFLICT (seller_id) DO NOTHING
            RETURNING name, seller_id;
            ",
        )
        .bind(new_sellers)
        .bind(seller_ids);

        let all: Vec<Seller> = mm.dbx().fetch_all(sqlx_query).await?;

        // Commit the transaction
        mm.dbx().commit_txn().await?;

        // endregion: --- Insert sellers

        Ok(all)
    }
}
