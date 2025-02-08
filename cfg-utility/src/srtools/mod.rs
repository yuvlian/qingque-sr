use serde::Deserialize;
use sr_proto::{
    AmountInfo, AvatarSkillTree, AvatarType, BattleAvatar, BattleBuff, BattleEquipment,
    BattleRelic, RelicAffix, SceneMonsterData, SceneMonsterWave, SceneMonsterWaveParam,
};
use std::collections::HashMap;
use std::fs;

macro_rules! trace {
    ($timed_id:expr; $($point:literal $level:literal);* $(;)?) => {{
        vec![
            $(
                AvatarSkillTree {
                    point_id: $timed_id + $point,
                    level: $level,
                }
            ),*
        ]
    }};
}

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
            .flat_map(|(i, av)| {
                av.buff_id_list.iter().map(move |&buff_id| {
                    // hardcode march tech to 3 stack
                    let dynamic_val = match buff_id {
                        122401..=122403 => HashMap::from([
                            (String::from("SkillIndex"), 0.0),
                            (String::from("#ADF_1"), 3.0),
                            (String::from("#ADF_2"), 3.0),
                        ]),
                        _ => HashMap::from([(String::from("SkillIndex"), 0.0)]),
                    };
                    BattleBuff {
                        id: buff_id,
                        owner_index: i as u32,
                        level: 1,
                        wave_flag: u32::MAX,
                        target_index_list: (0..=4).collect(),
                        dynamic_values: dynamic_val,
                    }
                })
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
                        monster_id,
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
            match avatar_id {
                // Remembrance characters
                // Fucking annoying...
                8007 | 8008 | 1402 => {
                    trace![
                        timed_id;
                        1 6;
                        2 10;
                        3 10;
                        4 10;
                        7 1;
                        101 1;
                        102 1;
                        103 1;
                        201 1;
                        202 1;
                        203 1;
                        204 1;
                        205 1;
                        206 1;
                        207 1;
                        208 1;
                        209 1;
                        210 1;
                        301 5;
                        302 5;
                    ]
                }
                _ => {
                    trace![
                        timed_id;
                        1 6;
                        2 10;
                        3 10;
                        4 10;
                        7 1;
                        101 1;
                        102 1;
                        103 1;
                        201 1;
                        202 1;
                        203 1;
                        204 1;
                        205 1;
                        206 1;
                        207 1;
                        208 1;
                        209 1;
                        210 1;
                    ]
                }
            }
        }

        fn create_lightcone(
            id: u32,
            rank: u32,
            level: u32,
            promotion: u32,
        ) -> Vec<BattleEquipment> {
            vec![BattleEquipment {
                id,
                rank,
                promotion,
                level,
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

                match parts.len() {
                    // if no step, high step
                    2 => (parts[0], parts[1], parts[1] * 2),
                    3 => (parts[0], parts[1], parts[2]),
                    _ => panic!("Invalid relic sub affix in config.json"),
                }
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
    // pub name: String,
    pub id: u32,
    pub hp: u32,
    pub sp: u32,
    pub level: u32,
    pub promotion: u32,
    pub rank: u32,
    pub lightcone: LightCone,
    pub relics: Vec<String>,
    // pub use_technique: bool,
    pub buff_id_list: Vec<u32>,
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
    // pub blessings: Vec<u32>,
}
