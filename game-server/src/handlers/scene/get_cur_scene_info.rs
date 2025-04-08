use configs::scene::SceneConfig;
use sr_proto::GetCurSceneInfoScRsp;

pub async fn handle(_: &[u8]) -> GetCurSceneInfoScRsp {
    let scene_cfg = SceneConfig::from_file("_configs_/scene.toml").await;

    GetCurSceneInfoScRsp {
        retcode: 0,
        scene: Some(scene_cfg.get_scene_info()),
    }
}
