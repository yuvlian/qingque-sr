use crate::utils::time;
use cfg_server::srtools::SrToolsConfig;
use net_msg::pb::{SceneBattleInfo, StartCocoonStageCsReq, StartCocoonStageScRsp};
use net_msg::{dec, Trait};

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let cfg = SrToolsConfig::from_file("_config/config.json");
    let req = dec!(StartCocoonStageCsReq, req);

    let battle_info = SceneBattleInfo {
        buff_list: cfg.get_battle_buffs(),
        battle_id: cfg.get_battle_id(),
        stage_id: cfg.get_stage_id(),
        cycle_count: cfg.get_cycle_count(),
        monster_wave_list: cfg.get_battle_waves(),
        battle_avatar_list: cfg.get_battle_avatars(),
        logic_random_seed: time::cur_timestamp_for_seed(),
        ..Default::default()
    };

    StartCocoonStageScRsp {
        retcode: 0,
        cocoon_id: req.cocoon_id,
        prop_entity_id: req.prop_entity_id,
        wave: 1,
        battle_info: Some(battle_info),
        ..Default::default()
    }
    .encode_to_vec()
}
