use sr_proto::MsgTrait;
use sr_proto::pb::PlayerLoginFinishScRsp;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    PlayerLoginFinishScRsp { retcode: 0 }.encode_to_vec()
}
