use cfg_server::scene::SceneConfig;
use net_msg::pb::GetCurSceneInfoScRsp;
use net_msg::Trait;

pub async fn handle(_: &[u8]) -> Vec<u8> {
    let scene_cfg = SceneConfig::from_file("_config/scene.toml");

    GetCurSceneInfoScRsp {
        retcode: 0,
        scene: Some(scene_cfg.get_scene_info()),
    }
    .encode_to_vec()
}
