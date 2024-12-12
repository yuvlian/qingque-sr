use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub avatar_config: Vec<AvatarConfig>,
    pub battle_config: BattleConfig,
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
