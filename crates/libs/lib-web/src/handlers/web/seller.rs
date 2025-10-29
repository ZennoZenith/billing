use crate::{error::Result, tera::render};
use axum::{extract::State, response::IntoResponse};
use lib_core::model::{self, ModelManager, seller::SellerBmc};
use tera::Context;
use tracing::debug;

pub async fn render_seller(
    State(mm): State<ModelManager>,
) -> Result<impl IntoResponse> {
    debug!("{:<12} - web_seller_handler", "HANDLER");

    let sellers = SellerBmc::get_all(&mm, None)
        .await
        .map_err(model::Error::from)?;

    let mut context = Context::new();

    context.insert("sellers", &sellers);
    render("routes/seller.html", &context).map(IntoResponse::into_response)
}
