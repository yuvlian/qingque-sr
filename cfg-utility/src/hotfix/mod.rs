use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Clone, Default)]
pub struct HotfixConfig {
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub lua_version: String,
}

#[derive(Deserialize, Default)]
pub struct GameVersion {
    #[serde(flatten)]
    pub versions: HashMap<String, HotfixConfig>,
}

impl GameVersion {
    pub fn from_file(file_path: &str) -> Self {
        let hotfix_json_data = fs::read_to_string(file_path);
        match hotfix_json_data {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn get_hotfix_for_version(&self, version: &Option<String>) -> HotfixConfig {
        match version {
            Some(v) => self.versions.get(v).cloned().unwrap_or_default(),
            None => HotfixConfig::default(),
        }
    }
}
