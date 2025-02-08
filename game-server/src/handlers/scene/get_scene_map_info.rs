use sr_proto::{GetSceneMapInfoCsReq, GetSceneMapInfoScRsp, SceneMapInfo, decode};

pub fn handle(req: &[u8]) -> GetSceneMapInfoScRsp {
    let req: GetSceneMapInfoCsReq = decode(req);

    GetSceneMapInfoScRsp {
        map_info_list: req
            .entry_id
            .iter()
            .map(|i| SceneMapInfo {
                entry_id: *i,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }
}
