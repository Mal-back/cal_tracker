use model::ModelManager;
use tokio::net::TcpListener;

use axum::{
    middleware,
    response::Html,
    routing::get,
    Router,
};

use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use web::mw_auth::mw_require_auth;

pub use self::error::{Error, Result};

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;

//#[cfg(test)]
mod _dev_utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Init tracing
    tracing_subscriber::fmt()
        .without_time() // comment for prod
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    _dev_utils::dev_init_db().await;

    let mm = ModelManager::new().await?;

    //let api_routes = web::routes_tickets::routes(mm.clone())
    //    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_login::routes(mm.clone()))
        .nest("/api", web::routes_user::routes(mm.clone()))
        //.nest("/api", api_routes)
        .layer(middleware::map_response(
            web::mw_res_map::main_response_mapper,
        ))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(web::routes_static::serve_dir());

    let listener = TcpListener::bind("backend:8443").await.unwrap();
    info!("---> Listening on {:?}", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
