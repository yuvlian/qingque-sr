use configs::server::ServerConfig;
use tokio::net::TcpListener;
use tracing::{error, info};

mod cmd;
mod conn;
mod packet;
mod util;

pub async fn start_game_server() -> tokio::io::Result<()> {
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
                info!("Client ({}) disconnected.", cl_addr);
            }
        });
    }
}
