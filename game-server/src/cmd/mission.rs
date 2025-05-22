use crate::packet::PacketSink;
use sr_proto::cmd::GET_MISSION_STATUS_SC_RSP;
use sr_proto::prost::Message;
use sr_proto::{GetMissionStatusCsReq, GetMissionStatusScRsp, Mission, MissionStatus};

pub async fn get_mission_status(req: &[u8], sink: &mut PacketSink) {
    let req = GetMissionStatusCsReq::decode(req).unwrap_or_default();

    let rsp = GetMissionStatusScRsp {
        finished_main_mission_id_list: req.main_mission_id_list,
        sub_mission_status_list: req
            .sub_mission_id_list
            .iter()
            .map(|id| Mission {
                id: *id,
                progress: 1,
                status: MissionStatus::MissionFinish.into(),
            })
            .collect(),
        ..Default::default()
    };

    sink.push_message(GET_MISSION_STATUS_SC_RSP, rsp);
}
