use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::handlers::web::fragmant::seller;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/seller/search", get(seller::search))
        .with_state(mm)
}
