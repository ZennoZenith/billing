use crate::{error::Result, middleware::mw_auth::CtxW, tera::render};
use axum::http::Uri;
use axum::response::{Html, IntoResponse};
use tera::Context;
use tracing::debug;

pub async fn fallback_render_not_found(uri: Uri) -> Result<Html<String>> {
    let mut context = Context::new();
    context.insert("uri", uri.to_string().as_str());
    render("error404.html", &context)
}

pub async fn render_login(_ctxw: Result<CtxW>) -> Result<impl IntoResponse> {
    debug!("{:<12} - web_login_handler", "HANDLER");

    // if ctxw.is_ok() {
    //     return Ok(Redirect::temporary("/dashboard").into_response());
    // }

    let context = Context::new();
    render("login.html", &context).map(IntoResponse::into_response)
}

pub async fn render_register(_ctxw: Result<CtxW>) -> Result<impl IntoResponse> {
    debug!("{:<12} - web_login_handler", "HANDLER");

    // if ctxw.is_ok() {
    //     return Ok(Redirect::temporary("/dashboard").into_response());
    // }

    let context = Context::new();
    render("register.html", &context).map(IntoResponse::into_response)
}
