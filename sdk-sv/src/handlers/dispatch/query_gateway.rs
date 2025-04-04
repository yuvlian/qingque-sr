use crate::ArcState;
use crate::app::request::QueryGatewayReq;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use db::sdk::hotfix::GatewayHotfix;
use sr_proto::{GateServer, Message};

// #[axum::debug_handler]
pub async fn get(
    State(state): State<ArcState>,
    Query(query): Query<QueryGatewayReq>,
) -> (StatusCode, String) {
    let gateway_hotfix = if state.env.auto_hotfix {
        match GatewayHotfix::get_or_fetch(&state.pool, &query.version, &query.dispatch_seed).await {
            Ok(v) => v,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string());
            }
        }
    } else {
        match GatewayHotfix::get_by_version(&state.pool, &query.version).await {
            Ok(Some(v)) => v,
            Err(_) | Ok(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string());
            }
        }
    };

    let rsp = GateServer {
        lua_url: gateway_hotfix.lua_url,
        asset_bundle_url: gateway_hotfix.asset_bundle_url,
        ex_resource_url: gateway_hotfix.ex_resource_url,
        ifix_url: gateway_hotfix.ifix_url,
        use_tcp: false,
        ip: state.env.game_sv_host.clone(),
        port: state.env.game_sv_port as u32,
        ifix_version: "0".to_string(),
        enable_design_data_version_update: true,
        enable_version_update: true,
        enable_upload_battle_log: true,
        network_diagnostic: true,
        close_redeem_code: true,
        enable_android_middle_package: true,
        enable_watermark: true,
        event_tracking_open: true,
        enable_cdn_ipv6: 1,
        enable_save_replay_file: true,
        ..Default::default()
    };

    (StatusCode::OK, rbase64::encode(&rsp.encode_to_vec()))
}
