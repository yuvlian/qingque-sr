use net_msg::pb::{PveBattleResultCsReq, PveBattleResultScRsp};
use net_msg::{dec, Trait};

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
