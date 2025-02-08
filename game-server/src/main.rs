use cfg_utility::server::ServerConfig;
use tokio::net::TcpListener;
use tracing::{error, info};

mod handlers;
mod network;
mod utils;

use network::conn;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let server_config = ServerConfig::from_file("_cfg/server.toml");

    tracing_subscriber::fmt().init();

    let addr = server_config.get_game_server_addr();

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    info!("Listening at {}", addr);

    loop {
        let (socket, client_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        info!("New connection: {}", client_addr);

        tokio::spawn(async move {
            if let Err(e) = conn::handle_connection(socket).await {
                error!("Connection error: {}", e);
            }
        });
    }
}
