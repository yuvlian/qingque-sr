use crate::packet::PacketSink;
use configs::avatar::AvatarConfig;
use sr_proto::cmd::{GET_AVATAR_DATA_SC_RSP, SET_AVATAR_ENHANCED_ID_SC_RSP};
use sr_proto::{GetAvatarDataScRsp, SetAvatarEnhancedIdScRsp};
use std::collections::HashMap;

pub async fn get_avatar_data(_req: &[u8], sink: &mut PacketSink) {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;
    let gender = cfg.get_trailblazer_gender();
    let (march, trailblazer) = cfg.get_multipath_data(gender);

    let rsp = GetAvatarDataScRsp {
        is_get_all: true,
        avatar_list: cfg.get_owned_avatars(),
        cur_avatar_path: HashMap::from([(1001, march.into()), (8001, trailblazer.into())]),
        ..Default::default()
    };

    sink.push_message(GET_AVATAR_DATA_SC_RSP, rsp);
}

pub async fn set_avatar_enhanced_id(_req: &[u8], sink: &mut PacketSink) {
    sink.push_message(
        SET_AVATAR_ENHANCED_ID_SC_RSP,
        SetAvatarEnhancedIdScRsp::default(),
    );
}
