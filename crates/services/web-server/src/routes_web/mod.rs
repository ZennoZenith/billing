use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::renders;

// region:    --- Modules
pub mod routes_static;

// endregion: --- Modules

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/login", get(renders::render_login))
        .route("/register", get(renders::render_register))
        .with_state(mm)
}
