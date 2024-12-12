use net_msg::pb::{StartCocoonStageCsReq, StartCocoonStageScRsp, SceneBattleInfo, SceneMonsterWave, SceneMonsterData, BattleAvatar, AvatarType, AmountInfo};
use net_msg::Trait;

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let dec = StartCocoonStageCsReq::decode(req).unwrap();

    let qingque = BattleAvatar {
        id: 1201,
        hp: 10000,
        level: 80,
        rank: 6,
        promotion: 6,
        avatar_type: AvatarType::AvatarFormalType.into(),
        sp: Some(AmountInfo {
            cur_amount: 0,
            max_amount: 10000,
        }),
        ..Default::default()
    };

    let wave = SceneMonsterWave {
        monster_list: vec![SceneMonsterData{
            monster_id: 3024020,
            ..Default::default()
        }],
        ..Default::default()
    };

    let battle_info = SceneBattleInfo {
        buff_list: Vec::with_capacity(0),
        battle_id: 1,
        stage_id: 201012311,
        logic_random_seed: u32::MAX,
        monster_wave_list:vec![wave],
        battle_avatar_list:vec![qingque],
        ..Default::default()
    };

    StartCocoonStageScRsp {
        retcode: 0,
        cocoon_id: dec.cocoon_id,
        prop_entity_id: dec.prop_entity_id,
        wave: 1,
        battle_info: Some(battle_info),
        ..Default::default()
    }
    .encode_to_vec()
}
