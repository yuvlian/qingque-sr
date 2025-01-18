use sr_proto::pb::{GetMissionStatusCsReq, GetMissionStatusScRsp, Mission, MissionStatus};
use sr_proto::{MsgTrait, dec};

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let req = dec!(GetMissionStatusCsReq, req);

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
    .encode_to_vec()
}
