use sr_proto::prost::Message;
use sr_proto::{PveBattleResultCsReq, PveBattleResultScRsp};

pub async fn handle(req: &[u8]) -> PveBattleResultScRsp {
    let req = PveBattleResultCsReq::decode(req).unwrap_or_default();

    PveBattleResultScRsp {
        retcode: 0,
        end_status: req.end_status,
        stage_id: req.stage_id,
        battle_id: req.battle_id,
        ..Default::default()
    }
}
