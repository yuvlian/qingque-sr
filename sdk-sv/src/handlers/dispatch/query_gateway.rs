use crate::ArcState;
use crate::app::db::hotfix::GatewayHotfix;
use crate::app::request::QueryGatewayReq;
use axum::extract::{Query, State};
use sr_proto::{GateServer, Message};

pub async fn get(State(state): State<ArcState>, Query(query): Query<QueryGatewayReq>) -> String {
    let gateway_hotfix = if state.env.auto_hotfix {
        GatewayHotfix::get_or_fetch(&state.pool, &query.version, &query.dispatch_seed)
            .await // unwrap_or_else because its lazy init
            .unwrap_or_else(|e| {
                tracing::warn!("GatewayHotfix is defaulting. Reason: {}", e);
                GatewayHotfix::default()
            })
    } else {
        GatewayHotfix::get_by_version(&state.pool, &query.version)
            .await
            .unwrap_or_else(|e| {
                tracing::warn!("GatewayHotfix is defaulting. Reason: {}", e);
                None
            })
            .unwrap_or_default()
    };

    let rsp = GateServer {
        lua_url: gateway_hotfix.lua_url,
        asset_bundle_url: gateway_hotfix.asset_bundle_url,
        ex_resource_url: gateway_hotfix.ex_resource_url,
        ifix_url: gateway_hotfix.ifix_url,
        use_tcp: true,
        ip: state.env.game_sv_host.clone(),
        port: state.env.game_sv_port as u32,
        ifix_version: "0".into(),
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

    rbase64::encode(&rsp.encode_to_vec())
}
