use net_msg::pb::{
    AmountInfo, AvatarSkillTree, AvatarType, BattleAvatar, BattleBuff, BattleEquipment,
    BattleRelic, RelicAffix, SceneMonsterData, SceneMonsterWave, SceneMonsterWaveParam,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct SrToolsConfig {
    pub avatar_config: Vec<AvatarConfig>,
    pub battle_config: BattleConfig,
}

impl Default for SrToolsConfig {
    fn default() -> Self {
        include!("./default_config.rs")
    }
}

impl SrToolsConfig {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path);
        match content {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn get_battle_id(&self) -> u32 {
        self.battle_config.battle_id
    }

    pub fn get_stage_id(&self) -> u32 {
        self.battle_config.stage_id
    }

    pub fn get_cycle_count(&self) -> u32 {
        self.battle_config.cycle_count
    }

    pub fn get_battle_buffs(&self) -> Vec<BattleBuff> {
        self.avatar_config
            .iter()
            .enumerate()
            .filter(|(_, av)| av.use_technique)
            .map(|(i, av)| BattleBuff {
                id: ((av.id * 100) + 1),
                owner_index: i as u32,
                level: 1,
                wave_flag: u32::MAX,
                target_index_list: (0..=4).collect(),
                dynamic_values: HashMap::from([(String::from("SkillIndex"), 0.0)]),
                ..Default::default()
            })
            .collect()
    }

    pub fn get_battle_waves(&self) -> Vec<SceneMonsterWave> {
        self.battle_config
            .monster_wave
            .iter()
            .map(|monster_ids| SceneMonsterWave {
                monster_list: monster_ids
                    .iter()
                    .map(|&monster_id| SceneMonsterData {
                        monster_id: monster_id,
                        ..Default::default()
                    })
                    .collect(),
                wave_param: Some(SceneMonsterWaveParam {
                    level: self.battle_config.monster_level,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect()
    }

    pub fn get_battle_avatars(&self) -> Vec<BattleAvatar> {
        fn create_max_trace(avatar_id: u32) -> Vec<AvatarSkillTree> {
            let timed_id = avatar_id * 1000;
            vec![
                AvatarSkillTree {
                    point_id: timed_id + 1,
                    level: 6,
                },
                AvatarSkillTree {
                    point_id: timed_id + 2,
                    level: 10,
                },
                AvatarSkillTree {
                    point_id: timed_id + 3,
                    level: 10,
                },
                AvatarSkillTree {
                    point_id: timed_id + 4,
                    level: 10,
                },
                AvatarSkillTree {
                    point_id: timed_id + 7,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 101,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 102,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 103,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 201,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 202,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 203,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 204,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 205,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 206,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 207,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 208,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 209,
                    level: 1,
                },
                AvatarSkillTree {
                    point_id: timed_id + 210,
                    level: 1,
                },
            ]
        }

        fn create_lightcone(
            id: u32,
            rank: u32,
            level: u32,
            promotion: u32,
        ) -> Vec<BattleEquipment> {
            vec![BattleEquipment {
                id: id,
                rank: rank,
                promotion: promotion,
                level: level,
            }]
        }

        fn create_relics(relic_strings: Vec<String>) -> Vec<BattleRelic> {
            type RelicId = u32;
            type RelicLv = u32;
            type RelicMAffix = u32;
            type RelicTotalSAffix = u32;
            // (SAffixId, SAffixCnt, SAffixStep)
            type RelicSAffix = (u32, u32, u32);

            fn parse_relic_string(
                relic_str: &str,
            ) -> (
                RelicId,
                RelicLv,
                RelicMAffix,
                RelicTotalSAffix,
                RelicSAffix,
                RelicSAffix,
                RelicSAffix,
                RelicSAffix,
            ) {
                let parts: Vec<&str> = relic_str.split(',').collect();

                let relic_id = parts[0]
                    .parse::<RelicId>()
                    .expect("Failed to parse RelicId");
                let relic_lv = parts[1]
                    .parse::<RelicLv>()
                    .expect("Failed to parse RelicLv");
                let relic_maffix = parts[2]
                    .parse::<RelicMAffix>()
                    .expect("Failed to parse RelicMAffix");
                let relic_total_saffix = parts[3]
                    .parse::<RelicTotalSAffix>()
                    .expect("Failed to parse RelicTotalSAffix");

                let saffix1: RelicSAffix = parse_saffix(parts[4]);
                let saffix2: RelicSAffix = parse_saffix(parts[5]);
                let saffix3: RelicSAffix = parse_saffix(parts[6]);
                let saffix4: RelicSAffix = parse_saffix(parts[7]);

                (
                    relic_id,
                    relic_lv,
                    relic_maffix,
                    relic_total_saffix,
                    saffix1,
                    saffix2,
                    saffix3,
                    saffix4,
                )
            }

            fn parse_saffix(s: &str) -> RelicSAffix {
                let parts: Vec<u32> = s
                    .split(':')
                    .map(|part| part.parse::<u32>().expect("Failed to parse RelicSAffix"))
                    .collect();
                if parts.len() != 3 {
                    panic!(
                        "Expected exactly 3 components in saffix, found {}",
                        parts.len()
                    );
                }
                (parts[0], parts[1], parts[2])
            }

            relic_strings
                .into_iter()
                .map(|rstring| {
                    let (
                        relic_id,
                        relic_lv,
                        relic_maffix,
                        _relic_total_saffix,
                        saffix1,
                        saffix2,
                        saffix3,
                        saffix4,
                    ) = parse_relic_string(&rstring);

                    let sub_affix_list = vec![
                        RelicAffix {
                            affix_id: saffix1.0,
                            cnt: saffix1.1,
                            step: saffix1.2,
                        },
                        RelicAffix {
                            affix_id: saffix2.0,
                            cnt: saffix2.1,
                            step: saffix2.2,
                        },
                        RelicAffix {
                            affix_id: saffix3.0,
                            cnt: saffix3.1,
                            step: saffix3.2,
                        },
                        RelicAffix {
                            affix_id: saffix4.0,
                            cnt: saffix4.1,
                            step: saffix4.2,
                        },
                    ];

                    BattleRelic {
                        id: relic_id,
                        level: relic_lv,
                        main_affix_id: relic_maffix,
                        sub_affix_list,
                        ..Default::default()
                    }
                })
                .collect()
        }

        fn create_energy(sp: u32) -> AmountInfo {
            AmountInfo {
                cur_amount: sp * 100,
                max_amount: 10000,
            }
        }

        self.avatar_config
            .iter()
            .enumerate()
            .map(|(i, av)| BattleAvatar {
                avatar_type: AvatarType::AvatarFormalType.into(),
                id: av.id,
                level: av.level,
                rank: av.rank,
                index: i as u32,
                skilltree_list: create_max_trace(av.id),
                equipment_list: create_lightcone(
                    av.lightcone.id,
                    av.lightcone.rank,
                    av.lightcone.level,
                    av.lightcone.promotion,
                ),
                hp: av.hp * 100,
                sp: Some(create_energy(av.sp)),
                promotion: av.promotion,
                // Clone to avoid borrowing issues
                relic_list: create_relics(av.relics.clone()),
                ..Default::default()
            })
            .collect()
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
