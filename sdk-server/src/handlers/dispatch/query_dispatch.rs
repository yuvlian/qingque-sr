use cfg_utility::server::ServerConfig;
use sr_proto::Message;
use sr_proto::{GlobalDispatchData, ServerData};

pub async fn handle() -> String {
    let server_config = ServerConfig::from_file("_cfg/server.toml");

    rbase64::encode(
        &GlobalDispatchData {
            retcode: 0,
            msg: String::from("OK"),
            server_list: vec![ServerData {
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
