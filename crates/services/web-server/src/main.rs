mod api;
mod error;

pub use self::error::{Error, Result};

use lib_web::{
    middleware::{
        mw_auth::mw_ctx_resolver, mw_req_stamp::mw_req_stamp_resolver,
        mw_res_map::mw_reponse_map,
    },
    web_config,
};

use axum::{Router, middleware};
use lib_web::model::ModelManager;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: Conditional compile only in debug build
    // -- FOR DEV ONLY
    {
        lib_web::_dev_utils::init_dev().await;
    }

    let mm = ModelManager::new().await?;

    let routes_all = Router::new()
        .merge(api::routes_login::routes(mm.clone()))
        .merge(api::routes_transaction::routes(mm.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(mw_req_stamp_resolver))
                .layer(CookieManagerLayer::new())
                .layer(middleware::from_fn_with_state(
                    mm.clone(),
                    mw_ctx_resolver,
                ))
                .layer(middleware::map_response(mw_reponse_map)), //
                                                                  // .layer(middleware::from_fn(
                                                                  //     lib_web::middleware::mw_auth::mw_ctx_require,
                                                                  // )),
        );

    // region:    --- Start Server
    // Note: For this block, ok to unwrap.
    let listener = TcpListener::bind(web_config().HOST_PORT).await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
