use net_msg::pb::{PveBattleResultCsReq, PveBattleResultScRsp};
use net_msg::Trait;

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let dec = PveBattleResultCsReq::decode(req).unwrap();

    PveBattleResultScRsp {
        retcode: 0,
        end_status: dec.end_status,
        stage_id: dec.stage_id,
        battle_id: dec.battle_id,
        ..Default::default()
    }
    .encode_to_vec()
}
