use cfg_utility::avatar::AvatarConfig;
use sr_proto::GetCurLineupDataScRsp;

pub fn handle(_: &[u8]) -> GetCurLineupDataScRsp {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    GetCurLineupDataScRsp {
        retcode: 0,
        lineup: Some(cfg.get_cur_lineup()),
    }
}
