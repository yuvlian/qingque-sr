use configs::avatar::AvatarConfig;
use sr_proto::GetAllLineupDataScRsp;

pub async fn handle(_: &[u8]) -> GetAllLineupDataScRsp {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;

    GetAllLineupDataScRsp {
        lineup_list: vec![cfg.get_cur_lineup()],
        cur_index: 0,
        retcode: 0,
    }
}
