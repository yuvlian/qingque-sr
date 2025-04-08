use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs;

#[derive(Deserialize, Clone, Default)]
pub struct HotfixConfig {
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub lua_version: String,
}

#[derive(Deserialize, Default)]
pub struct GameVersion(pub HashMap<String, HotfixConfig>);

impl GameVersion {
    pub async fn from_file(file_path: &str) -> Self {
        serde_json::from_str(&fs::read_to_string(file_path).await.unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn get_hotfix_by_version(&self, version: &Option<String>) -> HotfixConfig {
        match version {
            Some(v) => self.0.get(v).cloned().unwrap_or_default(),
            None => HotfixConfig::default(),
        }
    }
}
