mod handlers;
mod middleware;
mod router;

use axum::Router;
use configs::server::ServerConfig;
use middleware::log_requests;
use router::{auth_router, dispatch_router};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap_or(());

    tracing_subscriber::fmt().init();

    let addr = {
        let cfg = ServerConfig::from_file("_configs_/server.toml").await;
        format!("{}:{}", cfg.sdk_server_host, cfg.sdk_server_port)
    };

    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Listening on {}", addr);

    let app = Router::new()
        .merge(auth_router())
        .merge(dispatch_router())
        .layer(axum::middleware::from_fn(log_requests));

    axum::serve(listener, app).await.unwrap();
}
