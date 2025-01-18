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
    pub env_type: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            sdk_server_host: String::from("127.0.0.1"),
            sdk_server_port: 21000,
            game_server_host: String::from("127.0.0.1"),
            game_server_port: 59584,
            dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
            env_type: String::from("21"),
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

    pub fn get_sdk_socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.sdk_server_host, self.sdk_server_port)
            .parse()
            .expect("Failed parsing SDK SocketAddr")
    }
}
