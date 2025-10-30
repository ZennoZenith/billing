use crate::tera::render;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use tera::Context;

pub mod auth;
pub mod bill;
pub mod seller;
pub mod transaction;

pub mod fragmant;

pub async fn fallback(uri: Uri) -> (StatusCode, Response) {
    let body = format!("404 - Not found {uri}");

    let mut context = Context::new();
    context.insert("title", "Not Found");
    context.insert("message", &body);

    (
        StatusCode::NOT_FOUND,
        render("error404.html", &context).into_response(),
    )
}
