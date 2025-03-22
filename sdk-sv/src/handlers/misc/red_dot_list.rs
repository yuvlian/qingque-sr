use crate::app::{
    request::RedDotListReq,
    response::{IRsp, RedDotListRsp},
};
use axum::extract::Json as JsonEx;
use axum::response::Json;

pub async fn post(JsonEx(_req): JsonEx<RedDotListReq>) -> Json<IRsp<RedDotListRsp>> {
    Json(IRsp::<RedDotListRsp> {
        data: Some(RedDotListRsp::default()),
        ..Default::default()
    })
}
