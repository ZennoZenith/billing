use crate::error::{Error, Result};

use axum::extract::Query;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::{Json, extract::State};
use lib_core::model::transaction::{TransactionBmc, TransactionForCreate};
use lib_core::model::{self, ModelManager};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::debug;

pub async fn create_handler(
    State(mm): State<ModelManager>,
    payload_or_error: std::result::Result<
        Json<Vec<TransactionForCreate>>,
        JsonRejection,
    >,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_create_transaction_handler", "HANDLER");

    let payload = payload_or_error?.0;

    let transactions = TransactionBmc::create(&mm, payload)
        .await
        .map_err(model::Error::from)?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
            "transactions": transactions
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct GetTransactions {
    ids: Option<String>,
}

#[axum::debug_handler]
pub async fn get_handler(
    State(mm): State<ModelManager>,
    query: std::result::Result<Query<GetTransactions>, QueryRejection>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_get_transaction_handler", "HANDLER");

    let transaction_ids = query
        .map_err(|e| Error::QueryDeserialization(e.to_string()))?
        .0
        .ids
        .ok_or(Error::UnsupportedMedia)?
        .split(",")
        .map(ToOwned::to_owned)
        .collect();

    let transactions =
        TransactionBmc::get_by_transaction_ids(&mm, transaction_ids)
            .await
            .map_err(model::Error::from)?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
            "transactions": transactions
        }
    }));

    Ok(body)
}
// endregion: --- Register
