use axum::http::StatusCode;
use cfg_utility::server::ServerConfig;
use sr_proto::MsgTrait;
use sr_proto::pb::{GlobalDispatchData, ServerData};

pub async fn handle() -> (StatusCode, String) {
    let server_config = ServerConfig::from_file("_cfg/server.toml");

    let rsp = rbase64::encode(
        &GlobalDispatchData {
            retcode: 0,
            msg: String::from("OK"),
            server_list: vec![ServerData {
                name: String::from("smol"),
                display_name: String::from("smol"),
                title: String::from("smol"),
                env_type: String::from("2"),
                msg: String::from("OK"),
                dispatch_url: server_config.dispatch_url,
            }],
            top_sever_region_name: String::from("smol"),
            ..Default::default()
        }
        .encode_to_vec(),
    );

    (StatusCode::OK, rsp)
}
