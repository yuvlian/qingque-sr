use configs::avatar::AvatarConfig;
use sr_proto::GetBasicInfoScRsp;

pub async fn handle(_: &[u8]) -> GetBasicInfoScRsp {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;
    let gender = cfg.get_trailblazer_gender() as u32;

    GetBasicInfoScRsp {
        retcode: 0,
        is_gender_set: false,
        gender,
        ..Default::default()
    }
}
