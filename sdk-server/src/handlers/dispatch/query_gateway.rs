use axum::{extract::Query, http::StatusCode};
use cfg_utility::server::ServerConfig;
use cfg_utility::hotfix::GameVersion;
use sr_proto::pb::Gateserver;
use sr_proto::MsgTrait;

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

pub async fn handle(Query(q): Query<Gateway>) -> (StatusCode, String) {
    let server_config = ServerConfig::from_file("_cfg/server.toml");
    let game_version = GameVersion::from_file("_cfg/hotfix.json");

    let hotfix = game_version.get_hotfix_for_version(&q.version);

    let rsp = rbase64::encode(
        &Gateserver {
            // hotfix
            lua_url: hotfix.lua_url,
            lua_version: hotfix.lua_version,
            ex_resource_url: hotfix.ex_resource_url,
            asset_bundle_url: hotfix.asset_bundle_url,

            // we're not using kcp.
            use_tcp: true,
            ip: String::from(server_config.game_server_host()),
            port: server_config.game_server_port(),

            // let's just bruteforce the bool fields.
            unk1: true,
            unk2: true,
            unk3: true,
            unk4: true,
            unk5: true,
            unk6: true,
            unk7: true,
            // use_tcp was unk8
            // unk8: true,
            unk9: true,
            unk10: true,
            unk11: true,
            unk12: true,
            unk13: true,
            unk14: true,
            unk15: true,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    (StatusCode::OK, rsp)
}
