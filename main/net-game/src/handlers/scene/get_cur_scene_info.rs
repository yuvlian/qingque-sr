use net_msg::pb::scene_entity_info::EntityCase::{Actor, Prop};
use net_msg::pb::{
    AvatarType, GetCurSceneInfoScRsp, MotionInfo, SceneActorInfo, SceneEntityGroupInfo,
    SceneEntityInfo, SceneInfo, ScenePropInfo, Vector,
};
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    GetCurSceneInfoScRsp {
        retcode: 0,
        scene: Some(SceneInfo {
            plane_id: 20101,
            floor_id: 20101_001,
            entry_id: 20101_01,
            game_mode_type: 2,
            entity_group_list: vec![
                SceneEntityGroupInfo {
                    state: 1,
                    group_id: 0,
                    entity_list: vec![SceneEntityInfo {
                        group_id: 0,
                        inst_id: 0,
                        entity_id: 0,
                        entity_case: Some(Actor(SceneActorInfo {
                            avatar_type: AvatarType::AvatarFormalType.into(),
                            base_avatar_id: 1201,
                            map_layer: 2,
                            uid: 1,
                        })),
                        motion: Some(MotionInfo {
                            pos: Some(Vector {
                                x: 4480,
                                y: 19364,
                                z: -550,
                            }),
                            rot: Some(Vector { x: 0, y: 0, z: 0 }),
                        }),
                    }],
                    ..Default::default()
                },
                SceneEntityGroupInfo {
                    state: 1,
                    group_id: 19,
                    entity_list: vec![SceneEntityInfo {
                        group_id: 19,
                        inst_id: 300001,
                        entity_id: 228,
                        entity_case: Some(Prop(ScenePropInfo {
                            prop_id: 808,
                            prop_state: 1,
                            ..Default::default()
                        })),
                        motion: Some(MotionInfo {
                            pos: Some(Vector {
                                x: 4480,
                                y: 18354,
                                z: -570,
                            }),
                            rot: Some(Vector { x: 0, y: 0, z: 0 }),
                        }),
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
    }
    .encode_to_vec()
}
