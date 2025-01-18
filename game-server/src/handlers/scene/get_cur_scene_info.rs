use cfg_utility::scene::SceneConfig;
use sr_proto::pb::GetCurSceneInfoScRsp;
use sr_proto::MsgTrait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let scene_cfg = SceneConfig::from_file("_cfg/scene.toml");

    GetCurSceneInfoScRsp {
        retcode: 0,
        scene: Some(scene_cfg.get_scene_info()),
    }
    .encode_to_vec()
}
