use cfg_server::address::ServerConfig;
use tokio::io::Result as TokioResult;
use tokio::net::TcpListener;
use tracing::{error, info};

mod handlers;
mod network;
mod utils;

use network::conn;

#[tokio::main]
async fn main() -> TokioResult<()> {
    let server_config = ServerConfig::from_file("_config/server.toml");
    let to_bind = format!(
        "{}:{}",
        server_config.game_server_host(),
        server_config.game_server_port()
    );

    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind(&to_bind)
        .await
        .expect("Failed to bind to address");

    info!("Listening at {}", &to_bind);

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
