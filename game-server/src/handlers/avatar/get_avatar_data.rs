use cfg_utility::avatar::AvatarConfig;
use sr_proto::GetAvatarDataScRsp;

pub fn handle(_: &[u8]) -> GetAvatarDataScRsp {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    GetAvatarDataScRsp {
        is_get_all: true,
        avatar_list: cfg.get_owned_avatars(),
        retcode: 0,
        ..Default::default()
    }
}
