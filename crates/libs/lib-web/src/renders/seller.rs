use crate::{error::Result, middleware::mw_auth::CtxW, tera::render};
use axum::response::IntoResponse;
use tera::Context;
use tracing::debug;

pub async fn render_seller(_ctxw: Result<CtxW>) -> Result<impl IntoResponse> {
    debug!("{:<12} - web_seller_handler", "HANDLER");

    let context = Context::new();
    render("routes/login.html", &context).map(IntoResponse::into_response)
}
