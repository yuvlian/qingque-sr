use crate::utils::time;
use sr_proto::pb::{PlayerHeartBeatCsReq, PlayerHeartBeatScRsp};
use sr_proto::{MsgTrait, dec};

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
