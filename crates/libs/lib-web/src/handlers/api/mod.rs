use crate::error::{Error, Result};
use axum::http::Uri;

pub mod handlers_bill;
pub mod handlers_login;
pub mod handlers_seller;
pub mod handlers_transaction;

pub async fn fallback(uri: Uri) -> Result<()> {
    Err(Error::RouteNotExist(uri.to_string()))
}
