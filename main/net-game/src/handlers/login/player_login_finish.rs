use net_msg::pb::PlayerLoginFinishScRsp;
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    PlayerLoginFinishScRsp { retcode: 0 }.encode_to_vec()
}
