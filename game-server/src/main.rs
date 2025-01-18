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
    let socket_addr = format!(
        "{}:{}",
        server_config.game_server_host, server_config.game_server_port
    );

    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind(&socket_addr)
        .await
        .expect("Failed to bind to address");

    info!("Listening at {}", &socket_addr);

    loop {
        let (socket, addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        info!("New connection: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = conn::handle_connection(socket).await {
                error!("Connection error: {}", e);
            }
        });
    }
}
