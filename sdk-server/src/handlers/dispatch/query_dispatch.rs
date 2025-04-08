use configs::server::ServerConfig;
use sr_proto::prost::Message;
use sr_proto::{Dispatch, RegionInfo};

pub async fn handle() -> String {
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
