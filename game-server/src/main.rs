use configs::server::ServerConfig;
use tokio::net::TcpListener;
use tracing::{error, info};

mod handlers;
mod network;
mod utils;

use network::conn;

#[tokio::main(flavor = "current_thread")]
async fn main() -> tokio::io::Result<()> {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().expect("failed to enable ansi");

    tracing_subscriber::fmt().init();

    let addr = {
        let cfg = ServerConfig::from_file("_configs_/server.toml").await;
        format!("{}:{}", cfg.game_server_host, cfg.game_server_port)
    };

    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Listening on {}", addr);

    loop {
        let (socket, cl_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        info!("New connection: {}", cl_addr);

        tokio::spawn(async move {
            if let Err(e) = conn::handle_connection(socket).await {
                error!("Connection error: {}", e);
            }
        });
    }
}
