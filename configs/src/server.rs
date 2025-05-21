use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub sdk_server_host: String,
    pub sdk_server_port: u16,
    pub game_server_host: String,
    pub game_server_port: u32,
    pub dispatch_url: String,
    pub enable_auto_hotfix: bool,
    pub env_type: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            sdk_server_host: String::from("127.0.0.1"),
            sdk_server_port: 21000,
            game_server_host: String::from("127.0.0.1"),
            game_server_port: 23301,
            dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
            enable_auto_hotfix: false,
            env_type: String::from("2"),
        }
    }
}

impl ServerConfig {
    pub async fn from_file(file_path: &str) -> Self {
        toml::from_str(&fs::read_to_string(file_path).await.unwrap_or_default()).unwrap_or_default()
    }
}
