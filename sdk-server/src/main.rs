mod handlers;
mod middleware;
mod router;

use axum::Router;
use cfg_utility::server::ServerConfig;
use middleware::log_requests;
use router::{auth_router, dispatch_router};
use tracing::info;

#[tokio::main]
async fn main() {
    let server_config = ServerConfig::from_file("_cfg/server.toml");

    tracing_subscriber::fmt().init();

    let app = Router::new()
        .merge(auth_router())
        .merge(dispatch_router())
        .layer(axum::middleware::from_fn(log_requests));

    let addr = server_config.get_sdk_socket_addr();

    info!("Listening at {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind to address");
}
