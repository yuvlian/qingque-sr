use crate::packet::PacketSink;
use crate::util::cur_timestamp_for_seed;
use configs::srtools::SrToolsConfig;
use sr_proto::cmd::{PVE_BATTLE_RESULT_SC_RSP, START_COCOON_STAGE_SC_RSP};
use sr_proto::prost::Message;
use sr_proto::{
    PveBattleResultCsReq, PveBattleResultScRsp, SceneBattleInfo, StartCocoonStageCsReq,
    StartCocoonStageScRsp,
};

pub async fn pve_battle_result(req: &[u8], sink: &mut PacketSink) {
    let req = PveBattleResultCsReq::decode(req).unwrap_or_default();

    let rsp = PveBattleResultScRsp {
        retcode: 0,
        end_status: req.end_status,
        stage_id: req.stage_id,
        battle_id: req.battle_id,
        ..Default::default()
    };

    sink.push_message(PVE_BATTLE_RESULT_SC_RSP, rsp);
}

pub async fn start_cocoon_stage(req: &[u8], sink: &mut PacketSink) {
    let cfg = SrToolsConfig::from_file("_configs_/config.json").await;
    let req = StartCocoonStageCsReq::decode(req).unwrap_or_default();

    let battle_info = SceneBattleInfo {
        buff_list: cfg.get_battle_buffs(),
        battle_id: cfg.get_battle_id(),
        stage_id: cfg.get_stage_id(),
        rounds_limit: cfg.get_cycle_count(),
        monster_wave_list: cfg.get_battle_waves(),
        battle_avatar_list: cfg.get_battle_avatars(),
        logic_random_seed: cur_timestamp_for_seed(),
        ..Default::default()
    };

    let rsp = StartCocoonStageScRsp {
        cocoon_id: req.cocoon_id,
        prop_entity_id: req.prop_entity_id,
        wave: 1,
        battle_info: Some(battle_info),
        ..Default::default()
    };

    sink.push_message(START_COCOON_STAGE_SC_RSP, rsp);
}
