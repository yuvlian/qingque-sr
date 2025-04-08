use sr_proto::PlayerLoginFinishScRsp;

pub async fn handle(_: &[u8]) -> PlayerLoginFinishScRsp {
    PlayerLoginFinishScRsp::default()
}
