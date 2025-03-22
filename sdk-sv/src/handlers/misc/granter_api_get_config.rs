use crate::app::{
    request::GranterApiGetConfigReq,
    response::{GranterApiGetConfigRsp, IRsp},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_req): Query<GranterApiGetConfigReq>) -> Json<IRsp<GranterApiGetConfigRsp>> {
    Json(IRsp::<GranterApiGetConfigRsp> {
        data: Some(GranterApiGetConfigRsp {
            protocol: true,
            log_level: "INFO".to_string(),
            announce_url: "https://proxy1.neonteam.dev/sdk.hoyoverse.com/hkrpg/announcement/index.html?sdk_presentation_style=fullscreen&game=hkrpg&game_biz=hkrpg_global&sdk_screen_transparent=true&auth_appid=announcement&authkey_ver=1&version=2.34&sign_type=2#/".to_string(),
            app_name: "崩坏RPG".to_string(),
            enable_user_center: true,
            ..Default::default()
        }),
        ..Default::default()
    })
}
