use configs::server::ServerConfig;
use configs::logger::init_tracing;
use tokio::net::TcpListener;
use tracing::{error, info};

mod handlers;
mod network;
mod util;

use network::conn;

#[tokio::main(flavor = "current_thread")]
async fn main() -> tokio::io::Result<()> {
    init_tracing();

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
