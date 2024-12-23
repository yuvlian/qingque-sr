use cfg_server::avatar::AvatarConfig;
use net_msg::pb::GetAllLineupDataScRsp;
use net_msg::Trait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("_config/avatar.toml");

    GetAllLineupDataScRsp {
        lineup_list: vec![cfg.get_cur_lineup()],
        cur_index: 0,
        retcode: 0,
    }
    .encode_to_vec()
}
