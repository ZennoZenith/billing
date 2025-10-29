use crate::error::{Error, Result};
use axum::http::Uri;

pub async fn fallback_not_found(uri: Uri) -> Result<()> {
    Err(Error::RouteNotExist(uri.to_string()))
}
