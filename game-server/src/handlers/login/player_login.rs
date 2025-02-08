use crate::utils::time;
use sr_proto::{PlayerBasicInfo, PlayerLoginScRsp};

pub fn handle(_: &[u8]) -> PlayerLoginScRsp {
    PlayerLoginScRsp {
        basic_info: Some(PlayerBasicInfo {
            nickname: String::from("yulian"),
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
}
