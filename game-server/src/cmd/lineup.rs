use crate::packet::PacketSink;
use configs::avatar::AvatarConfig;
use sr_proto::cmd::{GET_ALL_LINEUP_DATA_SC_RSP, GET_CUR_LINEUP_DATA_SC_RSP};
use sr_proto::{GetAllLineupDataScRsp, GetCurLineupDataScRsp};

pub async fn get_cur_lineup_data(_req: &[u8], sink: &mut PacketSink) {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;
    let rsp = GetCurLineupDataScRsp {
        lineup: Some(cfg.get_cur_lineup()),
        ..Default::default()
    };

    sink.push_message(GET_CUR_LINEUP_DATA_SC_RSP, rsp);
}

pub async fn get_all_lineup_data(_req: &[u8], sink: &mut PacketSink) {
    let cfg = AvatarConfig::from_file("_configs_/avatar.toml").await;
    let rsp = GetAllLineupDataScRsp {
        lineup_list: vec![cfg.get_cur_lineup()],
        ..Default::default()
    };

    sink.push_message(GET_ALL_LINEUP_DATA_SC_RSP, rsp);
}
