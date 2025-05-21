use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct HotfixConfig {
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub ifix_url: String,
    pub lua_url: String,
    pub lua_version: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct GameVersion(pub HashMap<String, HotfixConfig>);

impl GameVersion {
    pub async fn from_file(file_path: &str) -> Self {
        serde_json::from_str(&fs::read_to_string(file_path).await.unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn get_hotfix_by_version(&self, version: &str) -> HotfixConfig {
        self.0.get(version).cloned().unwrap_or_default()
    }

    pub fn insert_hotfix_by_version(&mut self, version: &str, hotfix: HotfixConfig) {
        self.0.insert(version.to_owned(), hotfix);
    }

    pub async fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(file_path, json).await?;
        Ok(())
    }
}
