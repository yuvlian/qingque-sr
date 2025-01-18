use cfg_utility::avatar::AvatarConfig;
use sr_proto::pb::GetCurLineupDataScRsp;
use sr_proto::MsgTrait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    GetCurLineupDataScRsp {
        retcode: 0,
        lineup: Some(cfg.get_cur_lineup()),
    }
    .encode_to_vec()
}
