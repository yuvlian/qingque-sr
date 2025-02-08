use cfg_utility::scene::SceneConfig;
use sr_proto::GetCurSceneInfoScRsp;

pub fn handle(_: &[u8]) -> GetCurSceneInfoScRsp {
    let scene_cfg = SceneConfig::from_file("_cfg/scene.toml");

    GetCurSceneInfoScRsp {
        retcode: 0,
        scene: Some(scene_cfg.get_scene_info()),
    }
}
