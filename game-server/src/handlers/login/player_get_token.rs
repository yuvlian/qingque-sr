use sr_proto::PlayerGetTokenScRsp;

pub fn handle(_: &[u8]) -> PlayerGetTokenScRsp {
    PlayerGetTokenScRsp {
        msg: String::from("OK"),
        retcode: 0,
        uid: 1,
        ..Default::default()
    }
}
