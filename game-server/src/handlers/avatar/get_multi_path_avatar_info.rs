use cfg_utility::avatar::AvatarConfig;
use sr_proto::MsgTrait;
use sr_proto::pb::GetMultiPathAvatarInfoScRsp;
use std::collections::HashMap;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("_cfg/avatar.toml");

    let gender = cfg.get_trailblazer_gender();
    let (march, trailblazer) = cfg.get_multipath_data(gender);

    GetMultiPathAvatarInfoScRsp {
        retcode: 0,
        cur_avatar_path: HashMap::from([(1001, march.into()), (8001, trailblazer.into())]),
        ..Default::default()
    }
    .encode_to_vec()
}
