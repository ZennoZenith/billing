use crate::{error::Result, middleware::mw_auth::CtxW, tera::render};
use axum::response::IntoResponse;
use tera::Context;
use tracing::debug;

pub async fn render_login(_ctxw: Result<CtxW>) -> Result<impl IntoResponse> {
    debug!("{:<12} - web_login_handler", "HANDLER");

    // if ctxw.is_ok() {
    //     return Ok(Redirect::temporary("/dashboard").into_response());
    // }

    let context = Context::new();
    render("routes/login.html", &context).map(IntoResponse::into_response)
}

pub async fn render_register(_ctxw: Result<CtxW>) -> Result<impl IntoResponse> {
    let context = Context::new();
    render("routes/register.html", &context).map(IntoResponse::into_response)
}
