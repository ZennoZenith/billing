use axum::Router;
use lib_core::model::ModelManager;
use lib_web::handlers::api::fallback;

// region:    --- Modules
mod routes_login;
mod routes_transaction;

// endregion: --- Modules

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .merge(routes_transaction::routes(mm.clone()))
        .merge(routes_login::routes(mm))
        .fallback(fallback)
}
