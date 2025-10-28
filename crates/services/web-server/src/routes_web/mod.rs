use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::renders::{auth, seller};

// region:    --- Modules
pub mod routes_static;

// endregion: --- Modules

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/login", get(auth::render_login))
        .route("/register", get(auth::render_register))
        .route("/seller", get(seller::render_seller))
        .with_state(mm)
}
