use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::renders::auth;

// region:    --- Modules
pub mod routes_static;

// endregion: --- Modules

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/login", get(auth::render_login))
        .route("/register", get(auth::render_register))
        .with_state(mm)
}
