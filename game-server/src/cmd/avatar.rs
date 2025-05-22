use crate::packet::PacketSink;
use configs::avatar::AvatarConfig;
use sr_proto::GetAvatarDataScRsp;
use sr_proto::cmd::GET_AVATAR_DATA_SC_RSP;

pub async fn get_avatar_data(_req: &[u8], sink: &mut PacketSink) {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;

    let rsp = GetAvatarDataScRsp {
        is_get_all: true,
        avatar_list: cfg.get_owned_avatars(),
        ..Default::default()
    };

    sink.push_message(GET_AVATAR_DATA_SC_RSP, rsp);
}
