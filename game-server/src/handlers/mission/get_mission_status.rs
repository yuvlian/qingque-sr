use sr_proto::prost::Message;
use sr_proto::{GetMissionStatusCsReq, GetMissionStatusScRsp, Mission, MissionStatus};

pub async fn handle(req: &[u8]) -> GetMissionStatusScRsp {
    let req = GetMissionStatusCsReq::decode(req).unwrap_or_default();

    GetMissionStatusScRsp {
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
    }
}
