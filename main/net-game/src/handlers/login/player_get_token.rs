use net_msg::pb::PlayerGetTokenScRsp;
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    PlayerGetTokenScRsp {
        msg: String::from("OK"),
        retcode: 0,
        uid: 1,
        ..Default::default()
    }
    .encode_to_vec()
}
