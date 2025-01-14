use net_msg::pb::{
    AvatarType, MotionInfo, SceneActorInfo, SceneGroupInfo, SceneEntityInfo, SceneInfo,
    ScenePropInfo, Vector,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct SceneConfig {
    pub plane_id: u32,
    pub player: PlayerScene,
    pub calyx: CalyxScene,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            plane_id: 20101,
            player: PlayerScene {
                x: 4480,
                y: 19364,
                z: -550,
                base_avatar_id: 1201,
                map_layer: 2,
            },
            calyx: CalyxScene {
                x: 4480,
                y: 18354,
                z: -570,
                group_id: 19,
                inst_id: 300001,
                entity_id: 228,
                prop_id: 808,
            },
        }
    }
}

impl SceneConfig {
    pub fn from_file(file_path: &str) -> Self {
        let scene_toml_data = fs::read_to_string(file_path);
        match scene_toml_data {
            Ok(data) => toml::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn get_scene_info(&self) -> SceneInfo {
        SceneInfo {
            plane_id: self.plane_id,
            floor_id: (self.plane_id * 1000) + 1,
            entry_id: (self.plane_id * 100) + 1,
            game_mode_type: 2,
            scene_group_list: vec![
                SceneGroupInfo {
                    state: 1,
                    group_id: 0,
                    entity_list: vec![SceneEntityInfo {
                        group_id: 0,
                        inst_id: 0,
                        entity_id: 0,
                        actor: Some(SceneActorInfo {
                            avatar_type: AvatarType::AvatarFormalType.into(),
                            base_avatar_id: self.player.base_avatar_id,
                            map_layer: self.player.map_layer,
                            uid: 1,
                        }),
                        motion: Some(MotionInfo {
                            pos: Some(Vector {
                                x: self.player.x,
                                y: self.player.y,
                                z: self.player.z,
                            }),
                            rot: Some(Vector { x: 0, y: 0, z: 0 }),
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                SceneGroupInfo {
                    state: 1,
                    group_id: self.calyx.group_id,
                    entity_list: vec![SceneEntityInfo {
                        group_id: self.calyx.group_id,
                        inst_id: self.calyx.inst_id,
                        entity_id: self.calyx.entity_id,
                        prop: Some(ScenePropInfo {
                            prop_id: self.calyx.prop_id,
                            prop_state: 1,
                            ..Default::default()
                        }),
                        motion: Some(MotionInfo {
                            pos: Some(Vector {
                                x: self.calyx.x,
                                y: self.calyx.y,
                                z: self.calyx.z,
                            }),
                            rot: Some(Vector { x: 0, y: 0, z: 0 }),
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }
}

#[derive(Deserialize)]
pub struct PlayerScene {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub base_avatar_id: u32,
    pub map_layer: u32,
}

#[derive(Deserialize)]
pub struct CalyxScene {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub group_id: u32,
    pub inst_id: u32,
    pub entity_id: u32,
    pub prop_id: u32,
}
