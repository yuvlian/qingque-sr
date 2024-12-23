use crate::utils::time;
use net_msg::pb::{PlayerBasicInfo, PlayerLoginScRsp};
use net_msg::Trait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    PlayerLoginScRsp {
        basic_info: Some(PlayerBasicInfo {
            nickname: String::from("smol"),
            level: 10,
            exp: 0,
            stamina: 240,
            mcoin: 1,
            hcoin: 1,
            scoin: 1,
            world_level: 1,
        }),
        server_timestamp_ms: time::cur_timestamp_ms(),
        retcode: 0,
        stamina: 240,
        ..Default::default()
    }
    .encode_to_vec()
}
