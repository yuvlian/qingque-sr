use sr_proto::pb::{PveBattleResultCsReq, PveBattleResultScRsp};
use sr_proto::{MsgTrait, dec};

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let req = dec!(PveBattleResultCsReq, req);

    PveBattleResultScRsp {
        retcode: 0,
        end_status: req.end_status,
        stage_id: req.stage_id,
        battle_id: req.battle_id,
        ..Default::default()
    }
    .encode_to_vec()
}
