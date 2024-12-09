use axum::{extract::Query, http::StatusCode};
use cfg_server::address::ServerConfig;
use cfg_server::hotfix::GameVersion;
use net_msg::pb::GateServer;
use prost::Message;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Gateway {
    pub version: Option<String>,
    // t: Option<String>,
    // uid: Option<String>,
    // language_type: Option<i32>,
    // platform_type: Option<i32>,
    // dispatch_seed: Option<String>,
    // channel_id: Option<i32>,
    // sub_channel_id: Option<i32>,
    // is_need_url: Option<i32>,
    // game_version: Option<String>,
    // account_type: Option<i32>,
    // account_uid: Option<i64>,
}

pub async fn query_gateway(q: Query<Gateway>) -> (StatusCode, String) {
    let server_config = ServerConfig::from_file("server.toml");
    let game_version = GameVersion::from_file("hotfix.json");

    let hotfix = game_version.get_hotfix_for_version(&q.version);

    let rsp = rbase64::encode(
        &GateServer {
            // hotfix
            lua_bundle_version_update_url: hotfix.lua_url,
            lua_patch_version: hotfix.lua_version,
            design_data_bundle_version_update_url: hotfix.ex_resource_url,
            video_bundle_version_update_url: hotfix.asset_bundle_url,

            // gameserver
            retcode: 0,
            ip: String::from(server_config.game_server_host()),
            port: server_config.game_server_port(),
            use_tcp: true,

            // client bullshit
            watermark_enable: true,
            enable_video_bundle_version_update: true,
            close_redeem_code: true,
            forbid_recharge: true,
            enable_design_data_bundle_version_update: true,
            network_diagnostic: true,
            android_middle_package_enable: true,
            event_tracking_open: true,
            enable_save_replay_file: true,
            enable_upload_battle_log: true,
            ejcaokobhbg: true,
            nhehajgmjnj: true,
            ios_exam: true,
            mtp_switch: true,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    (StatusCode::OK, rsp)
}
