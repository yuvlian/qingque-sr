use crate::packet::PacketSink;
use configs::scene::SceneConfig;
use sr_proto::cmd::{
    GET_CUR_SCENE_INFO_SC_RSP, GET_SCENE_MAP_INFO_SC_RSP, SCENE_ENTITY_MOVE_SC_RSP,
};
use sr_proto::prost::Message;
use sr_proto::{
    GetCurSceneInfoScRsp, GetSceneMapInfoCsReq, GetSceneMapInfoScRsp, SceneEntityMoveScRsp,
    SceneMapInfo,
};

pub async fn get_cur_scene_info(_req: &[u8], sink: &mut PacketSink) {
    let scene_cfg = SceneConfig::from_file("_configs_/scene.toml").await;

    let rsp = GetCurSceneInfoScRsp {
        scene: Some(scene_cfg.get_scene_info()),
        ..Default::default()
    };

    sink.push_message(GET_CUR_SCENE_INFO_SC_RSP, rsp);
}

pub async fn get_scene_map_info(req: &[u8], sink: &mut PacketSink) {
    let req = GetSceneMapInfoCsReq::decode(req).unwrap_or_default();

    let rsp = GetSceneMapInfoScRsp {
        scene_map_info: req
            .entry_id_list
            .iter()
            .map(|i| SceneMapInfo {
                entry_id: *i,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };

    sink.push_message(GET_SCENE_MAP_INFO_SC_RSP, rsp);
}

pub async fn scene_entity_move(_req: &[u8], sink: &mut PacketSink) {
    sink.push_message(SCENE_ENTITY_MOVE_SC_RSP, SceneEntityMoveScRsp::default());
}
