use crate::{
    error::Result,
    extractors::{DatastarQuery, DatastarQueryError},
    tera::render_fragmant,
};
use axum::{extract::State, response::IntoResponse};
use lib_core::model::{self, ModelManager, seller::SellerBmc};
use serde::Deserialize;
use tera::Context;

#[derive(Debug, Deserialize)]
pub struct Search {
    search: String,
}

pub async fn search(
    State(mm): State<ModelManager>,
    query: std::result::Result<DatastarQuery<Search>, DatastarQueryError>,
) -> Result<impl IntoResponse> {
    let name = query?.0.search;

    dbg!(&name);

    let sellers = SellerBmc::search_by_name(&mm, &name, None)
        .await
        .map_err(model::Error::from)?;

    dbg!(&sellers);

    let mut context = Context::new();
    context.insert("sellers", &sellers);

    render_fragmant("fragmants/seller/searchResult.html", &context)
        .map(IntoResponse::into_response)
}
