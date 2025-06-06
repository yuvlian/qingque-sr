mod auth;
mod auto_hotfix;
mod dispatch;
mod logger;

use axum::Router;
use axum::middleware;
use configs::logger::init_tracing;
use configs::server::ServerConfig;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_tracing();

    let addr = {
        let cfg = ServerConfig::from_file("_configs_/server.toml").await;
        format!("{}:{}", cfg.sdk_server_host, cfg.sdk_server_port)
    };

    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Listening on {}", addr);

    let app = Router::new()
        .merge(auth::auth_router())
        .merge(dispatch::dispatch_router())
        .layer(middleware::from_fn(logger::log_requests));

    axum::serve(listener, app).await.unwrap();
}
