use configs::avatar::AvatarConfig;
use sr_proto::GetMultiPathAvatarInfoScRsp;
use std::collections::HashMap;

pub async fn handle(_: &[u8]) -> GetMultiPathAvatarInfoScRsp {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;

    let gender = cfg.get_trailblazer_gender();
    let (march, trailblazer) = cfg.get_multipath_data(gender);

    GetMultiPathAvatarInfoScRsp {
        retcode: 0,
        cur_avatar_path: HashMap::from([(1001, march.into()), (8001, trailblazer.into())]),
        ..Default::default()
    }
}
