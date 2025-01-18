use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub sdk_server_host: String,
    pub sdk_server_port: u16,
    pub game_server_host: String,
    pub game_server_port: u32,
    pub dispatch_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            sdk_server_host: String::from("127.0.0.1"),
            sdk_server_port: 21000,
            game_server_host: String::from("127.0.0.1"),
            game_server_port: 59584,
            dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
        }
    }
}

impl ServerConfig {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path);
        match content {
            Ok(data) => toml::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn sdk_to_listen_at(&self) -> SocketAddr {
        format!("{}:{}", self.sdk_server_host, self.sdk_server_port)
            .parse()
            .expect("Failed parsing SDK SocketAddr")
    }

    pub fn gateway_dispatch(&self) -> &str {
        &self.dispatch_url
    }

    pub fn game_server_host(&self) -> &str {
        &self.game_server_host
    }

    pub fn game_server_port(&self) -> u32 {
        self.game_server_port
    }
}
