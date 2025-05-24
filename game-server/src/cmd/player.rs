use crate::packet::PacketSink;
use crate::util::chat::PLAYER_UID;
use crate::util::cur_timestamp_ms;
use configs::avatar::AvatarConfig;
use sr_proto::cmd::{
    GET_BASIC_INFO_SC_RSP, PLAYER_GET_TOKEN_SC_RSP, PLAYER_HEART_BEAT_SC_RSP,
    PLAYER_LOGIN_FINISH_SC_RSP, PLAYER_LOGIN_SC_RSP,
};
use sr_proto::prost::Message;
use sr_proto::{
    ClientDownloadData, GetBasicInfoScRsp, PlayerBasicInfo, PlayerGetTokenScRsp,
    PlayerHeartBeatCsReq, PlayerHeartBeatScRsp, PlayerLoginFinishScRsp, PlayerLoginScRsp,
};

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

pub async fn player_get_token(_req: &[u8], sink: &mut PacketSink) {
    let rsp = PlayerGetTokenScRsp {
        msg: String::from("OK"),
        uid: PLAYER_UID,
        ..Default::default()
    };

    sink.push_message(PLAYER_GET_TOKEN_SC_RSP, rsp);
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

static SHOULD_SEND_LUA: AtomicBool = AtomicBool::new(false);
static LUA_CONTENT: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(String::with_capacity(2000)));
const MAGIC_TIME: u64 = 11112222;

fn insert_lua(content: &str) {
    let mut lua = LUA_CONTENT.lock().unwrap();
    lua.clear();
    lua.push_str(content);
}

fn clear_lua() {
    LUA_CONTENT.lock().unwrap().clear();
}

fn get_lua() -> String {
    LUA_CONTENT.lock().unwrap().clone()
}

pub async fn player_heart_beat(req: &[u8], sink: &mut PacketSink) {
    let req = PlayerHeartBeatCsReq::decode(req).unwrap_or_default();

    if req.client_time_ms == MAGIC_TIME {
        let file = req.lkjmjgdebee.unwrap();
        let file_content = file.value;
        insert_lua(&file_content);
        SHOULD_SEND_LUA.store(true, Ordering::Release);
        return;
    }

    if SHOULD_SEND_LUA.load(Ordering::Acquire) {
        let file_content = get_lua();
        let cur_time = cur_timestamp_ms();

        let rsp = PlayerHeartBeatScRsp {
            client_time_ms: req.client_time_ms,
            server_time_ms: cur_time,
            download_data: Some(ClientDownloadData {
                time: cur_time as i64,
                data: file_content.into(),
                ..Default::default()
            }),
            ..Default::default()
        };

        sink.push_message(PLAYER_HEART_BEAT_SC_RSP, rsp);
        SHOULD_SEND_LUA.store(false, Ordering::Release);
        clear_lua();
        return;
    }

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
