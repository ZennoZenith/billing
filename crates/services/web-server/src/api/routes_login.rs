use axum::Router;
use axum::routing::post;
use lib_web::{
    handlers::{handlers_fallback, handlers_login},
    model::ModelManager,
};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(handlers_login::api_login_handler))
        .route("/api/logoff", post(handlers_login::api_logoff_handler))
        .route("/api/register", post(handlers_login::api_register_handler))
        .fallback(handlers_fallback::fallback_not_found)
        .with_state(mm)
}
