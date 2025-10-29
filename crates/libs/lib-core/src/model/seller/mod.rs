use std::collections::HashSet;

use crate::model::ModelManager;
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
    pub seller_id: String,
    pub name: String,
}

pub struct SellerBmc;

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
            "select name from unnest($1::text[]) as t(name)
            where t.name not in (select name from seller);",
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
            "insert into seller (name, seller_id)
            select * from unnest(
                $1::text[],
                $2::text[]
            ) as t(name, seller_id)
            on conflict (seller_id) do nothing
            returning seller_id, name;
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

    pub async fn get_by_name(mm: &ModelManager, name: &str) -> Result<Seller> {
        let sqlx_query = sqlx::query_as::<_, Seller>(
            "select seller_id, name from seller where name = $1 limit 1;",
        )
        .bind(name);

        let user = mm.dbx().fetch_optional(sqlx_query).await?.ok_or(
            Error::SellerNotFound {
                name_or_id: format!("name: {}", name),
            },
        )?;

        Ok(user)
    }

    pub async fn get_by_seller_id(
        mm: &ModelManager,
        seller_id: &str,
    ) -> Result<Seller> {
        let sqlx_query = sqlx::query_as::<_, Seller>(
            "select seller_id, name from seller where seller_id = $1 limit 1;",
        )
        .bind(seller_id);

        let user = mm.dbx().fetch_optional(sqlx_query).await?.ok_or(
            Error::SellerNotFound {
                name_or_id: format!("seller_id: {}", seller_id),
            },
        )?;

        Ok(user)
    }

    pub async fn get_all(
        mm: &ModelManager,
        limit: Option<i32>,
    ) -> Result<Vec<Seller>> {
        let limit = match limit {
            Some(v) if (1..=50).contains(&v) => v,
            _ => 10,
        };

        let sqlx_query = sqlx::query_as::<_, Seller>(
            "select seller_id, name from seller limit $1;",
        )
        .bind(limit);

        let sellers = mm.dbx().fetch_all(sqlx_query).await?;

        Ok(sellers)
    }

    pub async fn search_by_name(
        mm: &ModelManager,
        name: &str,
        limit: Option<i32>,
    ) -> Result<Vec<Seller>> {
        let limit = match limit {
            Some(v) if (1..=50).contains(&v) => v,
            _ => 10,
        };

        let search_name = format!("%{}%", name);

        let sqlx_query = sqlx::query_as::<_, Seller>(
            "select seller_id, name from seller where name ilike $1 limit $2;",
        )
        .bind(search_name)
        .bind(limit);

        let sellers = mm.dbx().fetch_all(sqlx_query).await?;

        Ok(sellers)
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    pub type Result<T> = std::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>; // For tests.

    use super::*;
    use crate::_dev_utils;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let fx_name = "test_create_ok-seller-01";

        // -- Exec
        let sellers = SellerBmc::create(
            &mm,
            vec![SellerForCreate {
                name: fx_name.to_string(),
            }],
        )
        .await?;

        // -- Check
        let seller: Seller =
            SellerBmc::get_by_seller_id(&mm, &sellers[0].seller_id).await?;
        assert_eq!(seller.name, fx_name);

        // // -- Clean
        // SellerBmc::delete(&ctx, &mm, &seller.seller_id).await?;

        Ok(())
    }
}

// endregion: --- Tests
