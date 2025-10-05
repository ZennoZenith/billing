use axum::Router;
use axum::routing::post;
use lib_web::{
    handlers::{handlers_bill, handlers_seller, handlers_transaction},
    model::ModelManager,
};

pub fn routes(mm: ModelManager) -> Router {
    let transaction_routes = Router::new()
        .route("/", post(handlers_transaction::create_handler))
        .with_state(mm.clone());

    let bill_routes = Router::new()
        .route("/", post(handlers_bill::create_handler))
        .with_state(mm.clone());

    let seller_routes = Router::new()
        .route("/", post(handlers_seller::create_handler))
        .with_state(mm.clone());

    Router::new()
        .nest("/transaction", transaction_routes)
        .nest("/bill", bill_routes)
        .nest("/seller", seller_routes)
}
