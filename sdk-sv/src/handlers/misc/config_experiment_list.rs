use crate::app::{
    request::ConfigExperimentListReq,
    response::{ConfigExperiment, ConfigExperimentListRsp, IRsp, UserExperimentConfigs},
};
use axum::extract::Json as JsonEx;
use axum::response::Json;

pub async fn post(
    JsonEx(_req): JsonEx<ConfigExperimentListReq>,
) -> Json<IRsp<ConfigExperimentListRsp>> {
    Json(IRsp::<ConfigExperimentListRsp> {
        data: Some(vec![
            ConfigExperiment {
                code: 1000,
                r#type: 2,
                config_id: "200".to_string(),
                period_id: "6592_784".to_string(),
                version: "1".to_string(),
                configs: UserExperimentConfigs {
                    hoyopass_enable: Some("false".to_string()),
                    user_center_type: None,
                },
                scene_white_list: false,
                experiment_white_list: false,
            },
            ConfigExperiment {
                code: 1000,
                r#type: 2,
                config_id: "184".to_string(),
                period_id: "6554_748".to_string(),
                version: "1".to_string(),
                configs: UserExperimentConfigs {
                    hoyopass_enable: None,
                    user_center_type: Some("v1".to_string()),
                },
                scene_white_list: false,
                experiment_white_list: false,
            },
        ]),
        ..Default::default()
    })
}
