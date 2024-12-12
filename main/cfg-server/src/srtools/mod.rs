use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub avatar_config: Vec<AvatarConfig>,
    pub battle_config: BattleConfig,
}

impl Default for Config {
    fn default() -> Self {
        include!("./default_config.rs")
    }
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path);
        match content {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct AvatarConfig {
    pub name: String,
    pub id: u32,
    pub hp: u32,
    pub sp: u32,
    pub level: u32,
    pub promotion: u32,
    pub rank: u32,
    pub lightcone: LightCone,
    pub relics: Vec<String>,
    pub use_technique: bool,
}

#[derive(Deserialize)]
pub struct LightCone {
    pub id: u32,
    pub rank: u32,
    pub level: u32,
    pub promotion: u32,
}

#[derive(Deserialize)]
pub struct BattleConfig {
    pub battle_id: u32,
    pub stage_id: u32,
    pub cycle_count: u32,
    pub monster_wave: Vec<Vec<u32>>,
    pub monster_level: u32,
}