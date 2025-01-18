use cfg_utility::avatar::AvatarConfig;
use sr_proto::MsgTrait;
use sr_proto::pb::GetAvatarDataScRsp;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    GetAvatarDataScRsp {
        is_get_all: true,
        avatar_list: cfg.get_owned_avatars(),
        retcode: 0,
        ..Default::default()
    }
    .encode_to_vec()
}
