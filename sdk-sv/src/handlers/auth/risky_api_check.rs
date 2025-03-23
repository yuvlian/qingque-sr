use crate::app::{
    request::RiskyApiCheckReq,
    response::{IRsp, RiskyApiCheckRsp},
};
use axum::extract::Json as JsonEx;
use axum::response::Json;

pub async fn post(JsonEx(_req): JsonEx<RiskyApiCheckReq>) -> Json<IRsp<RiskyApiCheckRsp>> {
    Json(IRsp::<RiskyApiCheckRsp> {
        data: Some(RiskyApiCheckRsp {
            id: "9e54a9727a014ba4afd2cb2bb4347fe3".to_string(),
            action: "ACTION_NONE".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    })
}
