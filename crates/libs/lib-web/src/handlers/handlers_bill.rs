use crate::error::Result;
use crate::model::ModelManager;
use crate::model::bill::{BillBmc, BillForCreate};

use axum::extract::rejection::JsonRejection;
use axum::{Json, extract::State};
use serde_json::{Value, json};
use tracing::debug;

pub async fn create_handler(
    State(mm): State<ModelManager>,
    payload_or_error: std::result::Result<Json<BillForCreate>, JsonRejection>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_create_bill_handler", "HANDLER");

    let payload = payload_or_error?.0;

    let bill_id = BillBmc::create(&mm, payload).await?;

    // Create the success body.
    let body = Json(json!({
     "result": {
      "success": true,
      "billId": bill_id
     }
    }));

    Ok(body)
}
// endregion: --- Register
