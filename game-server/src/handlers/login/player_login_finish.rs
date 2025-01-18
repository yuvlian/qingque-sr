use sr_proto::pb::PlayerLoginFinishScRsp;
use sr_proto::MsgTrait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    PlayerLoginFinishScRsp { retcode: 0 }.encode_to_vec()
}
