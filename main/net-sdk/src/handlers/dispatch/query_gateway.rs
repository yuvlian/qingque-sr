use axum::{extract::Query, http::StatusCode};
use cfg_server::address::ServerConfig;
use cfg_server::hotfix::GameVersion;
use net_msg::pb::GateServer;
use net_msg::Trait;

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

pub async fn handle(q: Query<Gateway>) -> (StatusCode, String) {
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
            ip: String::from(server_config.game_server_host()),
            port: server_config.game_server_port(),
            use_tcp: true,

            // client bullshit
            retcode: 0,
            watermark_enable: false,
            enable_video_bundle_version_update: true,
            close_redeem_code: true,
            forbid_recharge: true,
            enable_design_data_bundle_version_update: true,
            network_diagnostic: false,
            android_middle_package_enable: false,
            event_tracking_open: false,
            enable_save_replay_file: true,
            enable_upload_battle_log: true,
            ejcaokobhbg: true,
            nhehajgmjnj: false,
            ios_exam: false,
            mtp_switch: false,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    (StatusCode::OK, rsp)
}
