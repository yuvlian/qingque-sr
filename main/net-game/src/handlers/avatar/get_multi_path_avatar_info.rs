use cfg_server::avatar::AvatarConfig;
use net_msg::pb::{Gender, GetMultiPathAvatarInfoScRsp};
use net_msg::Trait;
use std::collections::HashMap;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("avatar.toml");

    let gender = cfg.get_trailblazer_gender();
    let tb_id = match gender {
        Gender::Woman => 8002,
        Gender::Man => 8001,
        _ => 0,
    };

    let (march, trailblazer) = cfg.get_multipath_data(gender);

    GetMultiPathAvatarInfoScRsp {
        retcode: 0,
        cur_multi_path_avatar_type_map: HashMap::from([
            (1001, march.into()),
            (tb_id, trailblazer.into()),
        ]),
        ..Default::default()
    }
    .encode_to_vec()
}
