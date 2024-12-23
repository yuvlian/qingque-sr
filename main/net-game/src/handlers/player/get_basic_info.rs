use cfg_server::avatar::AvatarConfig;
use net_msg::pb::GetBasicInfoScRsp;
use net_msg::Trait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("_config/avatar.toml");
    let gender = cfg.get_trailblazer_gender() as u32;

    GetBasicInfoScRsp {
        retcode: 0,
        is_gender_set: false,
        gender: gender,
        ..Default::default()
    }
    .encode_to_vec()
}
