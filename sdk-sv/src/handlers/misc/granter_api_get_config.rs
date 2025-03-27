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
            announce_url: "https://f2pqingque.github.io/what.html/".to_string(),
            app_name: "崩坏RPG".to_string(),
            enable_user_center: false,
            ..Default::default()
        }),
        ..Default::default()
    })
}
