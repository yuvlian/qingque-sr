use configs::avatar::AvatarConfig;
use sr_proto::GetCurLineupDataScRsp;

pub async fn handle(_: &[u8]) -> GetCurLineupDataScRsp {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;

    GetCurLineupDataScRsp {
        retcode: 0,
        lineup: Some(cfg.get_cur_lineup()),
    }
}
