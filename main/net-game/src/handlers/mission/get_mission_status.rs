use net_msg::pb::{GetMissionStatusCsReq, GetMissionStatusScRsp, Mission, MissionStatus};
use net_msg::Trait;

pub async fn handle(req: &[u8]) -> Vec<u8> {
    let dec = GetMissionStatusCsReq::decode(req).unwrap();

    GetMissionStatusScRsp {
        finished_main_mission_id_list: dec.main_mission_id_list,
        sub_mission_status_list: dec
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
