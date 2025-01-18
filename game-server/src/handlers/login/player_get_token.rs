use sr_proto::MsgTrait;
use sr_proto::pb::PlayerGetTokenScRsp;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    PlayerGetTokenScRsp {
        msg: String::from("OK"),
        retcode: 0,
        uid: 1,
        ..Default::default()
    }
    .encode_to_vec()
}
