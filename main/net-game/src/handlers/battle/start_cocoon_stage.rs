use cfg_server::srtools::Config;
use net_msg::pb::{SceneBattleInfo, StartCocoonStageCsReq, StartCocoonStageScRsp};
use net_msg::Trait;

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let cfg = Config::from_file("config.json");
    let dec = StartCocoonStageCsReq::decode(req).unwrap();

    let battle_info = SceneBattleInfo {
        buff_list: cfg.get_battle_buffs(),
        battle_id: cfg.get_battle_id(),
        stage_id: cfg.get_stage_id(),
        cycle_count: cfg.get_cycle_count(),
        monster_wave_list: cfg.get_battle_waves(),
        battle_avatar_list: cfg.get_battle_avatars(),
        logic_random_seed: u32::MAX,
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
