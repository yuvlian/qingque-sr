use sr_proto::{GetSceneMapInfoCsReq, GetSceneMapInfoScRsp, SceneMapInfo};
use sr_proto::prost::Message;

pub async fn handle(req: &[u8]) -> GetSceneMapInfoScRsp {
    let req = GetSceneMapInfoCsReq::decode(req).unwrap_or_default();

    GetSceneMapInfoScRsp {
        scene_map_info: req
            .entry_id_list
            .iter()
            .map(|i| SceneMapInfo {
                entry_id: *i,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }
}
