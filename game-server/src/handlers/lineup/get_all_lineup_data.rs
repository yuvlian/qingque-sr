use cfg_utility::avatar::AvatarConfig;
use sr_proto::MsgTrait;
use sr_proto::pb::GetAllLineupDataScRsp;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    GetAllLineupDataScRsp {
        lineup_list: vec![cfg.get_cur_lineup()],
        cur_index: 0,
        retcode: 0,
    }
    .encode_to_vec()
}
