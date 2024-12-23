use net_msg::pb::PlayerLoginFinishScRsp;
use net_msg::Trait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    PlayerLoginFinishScRsp { retcode: 0 }.encode_to_vec()
}
