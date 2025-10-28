use crate::{error::Result, tera::render};
use axum::http::Uri;
use axum::response::Html;
use tera::Context;

pub mod auth;
pub mod seller;

pub async fn fallback_render_not_found(uri: Uri) -> Result<Html<String>> {
    let mut context = Context::new();
    context.insert("uri", uri.to_string().as_str());
    render("error404.html", &context)
}
