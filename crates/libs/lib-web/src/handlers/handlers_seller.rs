use crate::error::Result;
use crate::model::ModelManager;
use crate::model::seller::{SellerBmc, SellerForCreate};

use axum::extract::rejection::JsonRejection;
use axum::{Json, extract::State};
use serde_json::{Value, json};
use tracing::debug;

pub async fn create_handler(
    State(mm): State<ModelManager>,
    payload_or_error: std::result::Result<
        Json<Vec<SellerForCreate>>,
        JsonRejection,
    >,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    let payload = payload_or_error?.0;

    let sellers = SellerBmc::create(&mm, payload).await?;

    // Create the success body.
    let body = Json(json!({
     "result": {
      "success": true,
      "sellers": sellers
     }
    }));

    Ok(body)
}
// endregion: --- Register
