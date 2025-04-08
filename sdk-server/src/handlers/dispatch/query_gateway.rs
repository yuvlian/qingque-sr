use axum::extract::Query;
use configs::hotfix::GameVersion;
use configs::server::ServerConfig;
use sr_proto::GateServer;
use sr_proto::prost::Message;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Gateway {
    pub version: Option<String>,
}

pub async fn handle(Query(q): Query<Gateway>) -> String {
    let server_config = ServerConfig::from_file("_configs_/server.toml").await;
    let game_version = GameVersion::from_file("_configs_/hotfix.json").await;
    let hotfix = game_version.get_hotfix_by_version(&q.version);

    rbase64::encode(
        &GateServer {
            // hotfix
            lua_url: hotfix.lua_url,
            ex_resource_url: hotfix.ex_resource_url,
            asset_bundle_url: hotfix.asset_bundle_url,
            mdk_res_version: hotfix.lua_version,
            ifix_version: String::from("0"),

            // we're not using kcp.
            use_tcp: true,
            ip: server_config.game_server_host,
            port: server_config.game_server_port,

            // misc
            enable_version_update: true,
            enable_design_data_version_update: true,
            enable_save_replay_file: true,
            enable_upload_battle_log: true,
            enable_watermark: true,
            event_tracking_open: true,
            ..Default::default()
        }
        .encode_to_vec(),
    )
}
