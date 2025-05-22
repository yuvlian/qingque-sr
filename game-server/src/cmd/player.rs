use crate::packet::PacketSink;
use crate::util::cur_timestamp_ms;
use configs::avatar::AvatarConfig;
use sr_proto::cmd::{
    GET_BASIC_INFO_SC_RSP, GET_MULTI_PATH_AVATAR_INFO_SC_RSP, PLAYER_GET_TOKEN_SC_RSP,
    PLAYER_HEART_BEAT_SC_RSP, PLAYER_LOGIN_FINISH_SC_RSP, PLAYER_LOGIN_SC_RSP,
};
use sr_proto::prost::Message;
use sr_proto::{
    GetBasicInfoScRsp, GetMultiPathAvatarInfoScRsp, PlayerBasicInfo, PlayerGetTokenScRsp,
    PlayerHeartBeatCsReq, PlayerHeartBeatScRsp, PlayerLoginFinishScRsp, PlayerLoginScRsp,
};
use std::collections::HashMap;

pub async fn get_basic_info(_req: &[u8], sink: &mut PacketSink) {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;
    let gender = cfg.get_trailblazer_gender() as u32;
    let rsp = GetBasicInfoScRsp {
        is_gender_set: false,
        gender,
        ..Default::default()
    };

    sink.push_message(GET_BASIC_INFO_SC_RSP, rsp);
}

pub async fn get_multi_path_avatar_info(_req: &[u8], sink: &mut PacketSink) {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;
    let gender = cfg.get_trailblazer_gender();
    let (march, trailblazer) = cfg.get_multipath_data(gender);
    let rsp = GetMultiPathAvatarInfoScRsp {
        cur_avatar_path: HashMap::from([(1001, march.into()), (8001, trailblazer.into())]),
        ..Default::default()
    };

    sink.push_message(GET_MULTI_PATH_AVATAR_INFO_SC_RSP, rsp);
}

pub async fn player_get_token(_req: &[u8], sink: &mut PacketSink) {
    let rsp = PlayerGetTokenScRsp {
        msg: String::from("OK"),
        uid: 800047331,
        ..Default::default()
    };

    sink.push_message(PLAYER_GET_TOKEN_SC_RSP, rsp);
}

pub async fn player_heart_beat(req: &[u8], sink: &mut PacketSink) {
    let req = PlayerHeartBeatCsReq::decode(req).unwrap_or_default();

    let rsp = PlayerHeartBeatScRsp {
        client_time_ms: req.client_time_ms,
        server_time_ms: cur_timestamp_ms(),
        ..Default::default()
    };

    sink.push_message(PLAYER_HEART_BEAT_SC_RSP, rsp);
}

pub async fn player_login(_req: &[u8], sink: &mut PacketSink) {
    let rsp = PlayerLoginScRsp {
        basic_info: Some(PlayerBasicInfo {
            nickname: String::from("yuvlian"),
            level: 10,
            stamina: 240,
            world_level: 1,
            ..Default::default()
        }),
        server_timestamp_ms: cur_timestamp_ms(),
        stamina: 240,
        ..Default::default()
    };

    sink.push_message(PLAYER_LOGIN_SC_RSP, rsp);
}

pub async fn player_login_finish(_req: &[u8], sink: &mut PacketSink) {
    sink.push_message(
        PLAYER_LOGIN_FINISH_SC_RSP,
        PlayerLoginFinishScRsp::default(),
    );
}
