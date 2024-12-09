use axum::http::StatusCode;
use cfg_server::address::ServerConfig;
use net_msg::pb::{Dispatch, RegionInfo};
use prost::Message;

pub async fn query_dispatch() -> (StatusCode, String) {
    let server_config = ServerConfig::from_file("server.toml");

    let rsp = rbase64::encode(
        &Dispatch {
            retcode: 0,
            msg: String::from("OK"),
            region_list: vec![RegionInfo {
                name: String::from("smol"),
                display_name: String::from("smol"),
                title: String::from("smol"),
                env_type: String::from("2"),
                msg: String::from("OK"),
                dispatch_url: String::from(server_config.gateway_dispatch()),
            }],
            ..Default::default()
        }
        .encode_to_vec(),
    );

    (StatusCode::OK, rsp)
}
