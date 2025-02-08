use cfg_utility::avatar::AvatarConfig;
use sr_proto::GetAllLineupDataScRsp;

pub fn handle(_: &[u8]) -> GetAllLineupDataScRsp {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    GetAllLineupDataScRsp {
        lineup_list: vec![cfg.get_cur_lineup()],
        cur_index: 0,
        retcode: 0,
    }
}
