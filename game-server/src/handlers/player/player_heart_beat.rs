use crate::utils::time;
use sr_proto::prost::Message;
use sr_proto::{PlayerHeartBeatCsReq, PlayerHeartBeatScRsp};

pub async fn handle(req: &[u8]) -> PlayerHeartBeatScRsp {
    let req = PlayerHeartBeatCsReq::decode(req).unwrap_or_default();

    PlayerHeartBeatScRsp {
        download_data: None,
        client_time_ms: req.client_time_ms,
        server_time_ms: time::cur_timestamp_ms(),
        retcode: 0,
    }
}
