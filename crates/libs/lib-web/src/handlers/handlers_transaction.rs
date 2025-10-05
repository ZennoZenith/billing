use crate::error::Result;
use crate::model::ModelManager;
use crate::model::transaction::{TransactionBmc, TransactionForCreate};

use axum::extract::rejection::JsonRejection;
use axum::{Json, extract::State};
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

    TransactionBmc::create(&mm, payload).await?;

    // Create the success body.
    let body = Json(json!({
     "result": {
      "success": true
     }
    }));

    Ok(body)
}
// endregion: --- Register
