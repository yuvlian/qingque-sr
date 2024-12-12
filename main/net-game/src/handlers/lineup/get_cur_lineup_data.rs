use cfg_server::avatar::AvatarConfig;
use net_msg::pb::GetCurLineupDataScRsp;
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("avatar.toml");

    GetCurLineupDataScRsp {
        retcode: 0,
        lineup: Some(cfg.get_cur_lineup()),
    }
    .encode_to_vec()
}
