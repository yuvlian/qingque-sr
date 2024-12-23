mod handlers;
mod middleware;
mod router;

use axum::Router;
use cfg_server::address::ServerConfig;
use middleware::log_requests;
use router::{auth_router, dispatch_router};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    let server_config = ServerConfig::from_file("_config/server.toml");

    tracing_subscriber::fmt().init();

    let app = Router::new()
        .merge(auth_router())
        .merge(dispatch_router())
        .layer(axum::middleware::from_fn(log_requests));

    let addr = SocketAddr::from(server_config.sdk_to_listen_at());

    info!("Listening at {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind to address");
}
