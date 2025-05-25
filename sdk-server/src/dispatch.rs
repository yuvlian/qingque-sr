use crate::auto_hotfix;
use axum::{Router, extract::Query, routing::get};
use configs::hotfix::{GameVersion, HotfixConfig};
use configs::server::ServerConfig;
use serde::Deserialize;
use sr_proto::prost::Message;
use sr_proto::{Dispatch, GateServer, RegionInfo};

pub fn dispatch_router() -> Router<()> {
    let mut router = Router::new();
    router = router.route(ON_QUERY_DISPATCH_ROUTE, get(on_query_dispatch));
    router = router.route(ON_QUERY_GATEWAY_ROUTE, get(on_query_gateway));
    router
}

const ON_QUERY_DISPATCH_ROUTE: &str = "/query_dispatch";
async fn on_query_dispatch() -> String {
    let server_config = ServerConfig::from_file("_configs_/server.toml").await;

    rbase64::encode(
        &Dispatch {
            retcode: 0,
            msg: String::from("OK"),
            region_list: vec![RegionInfo {
                name: String::from("yulian"),
                display_name: String::from("yulian"),
                title: String::from("yulian"),
                env_type: server_config.env_type,
                msg: String::from("OK"),
                dispatch_url: server_config.dispatch_url,
            }],
            top_sever_region_name: String::from("yulian"),
            ..Default::default()
        }
        .encode_to_vec(),
    )
}

#[derive(Deserialize)]
struct Gateway {
    version: Option<String>,
    dispatch_seed: Option<String>,
}

const ON_QUERY_GATEWAY_ROUTE: &str = "/query_gateway";
async fn on_query_gateway(Query(q): Query<Gateway>) -> String {
    let server_config = ServerConfig::from_file("_configs_/server.toml").await;
    let version = q.version.unwrap_or_default();
    let dispatch_seed = q.dispatch_seed.unwrap_or_default();

    let hotfix = if !server_config.enable_auto_hotfix {
        GameVersion::from_file("_configs_/hotfix.json")
            .await
            .get_hotfix_by_version(&version)
    } else {
        match auto_hotfix::try_get_and_update(&version, &dispatch_seed).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("failed auto hotfix: {}", e);
                HotfixConfig::default()
            }
        }
    };

    rbase64::encode(
        &GateServer {
            // hotfix
            lua_url: hotfix.lua_url,
            ifix_url: hotfix.ifix_url,
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
