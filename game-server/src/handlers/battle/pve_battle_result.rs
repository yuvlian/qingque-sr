use sr_proto::{PveBattleResultCsReq, PveBattleResultScRsp, decode};

pub fn handle(req: &[u8]) -> PveBattleResultScRsp {
    let req: PveBattleResultCsReq = decode(req);

    PveBattleResultScRsp {
        retcode: 0,
        end_status: req.end_status,
        stage_id: req.stage_id,
        battle_id: req.battle_id,
        ..Default::default()
    }
}
