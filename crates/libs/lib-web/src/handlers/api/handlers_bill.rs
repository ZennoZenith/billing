use crate::error::Result;

use axum::extract::rejection::JsonRejection;
use axum::{Json, extract::State};
use lib_core::model::bill::{BillBmc, BillForCreate};
use lib_core::model::{self, ModelManager};
use serde_json::{Value, json};
use tracing::debug;

pub async fn create_handler(
    State(mm): State<ModelManager>,
    payload_or_error: std::result::Result<Json<BillForCreate>, JsonRejection>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_create_bill_handler", "HANDLER");

    let payload = payload_or_error?.0;

    let bill_id = BillBmc::create(&mm, payload)
        .await
        .map_err(model::Error::from)?;

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
