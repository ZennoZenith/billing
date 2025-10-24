use axum::Router;
use axum::routing::post;
use lib_core::model::ModelManager;
use lib_web::handlers::{handlers_bill, handlers_seller, handlers_transaction};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/transaction",
            post(handlers_transaction::create_handler)
                .get(handlers_transaction::get_handler),
        )
        .route("/bill", post(handlers_bill::create_handler))
        .route("/seller", post(handlers_seller::create_handler))
        .with_state(mm.clone())
}
