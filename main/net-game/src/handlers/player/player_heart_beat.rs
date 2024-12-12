use crate::utils::time;
use net_msg::pb::{PlayerHeartBeatCsReq, PlayerHeartBeatScRsp};
use net_msg::{dec, Trait};

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let req = dec!(PlayerHeartBeatCsReq, req);

    PlayerHeartBeatScRsp {
        download_data: None,
        client_time_ms: req.client_time_ms,
        server_time_ms: time::cur_timestamp_ms(),
        retcode: 0,
    }
    .encode_to_vec()
}
