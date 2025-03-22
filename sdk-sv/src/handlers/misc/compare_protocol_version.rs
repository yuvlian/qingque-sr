use crate::app::{
    request::CompareProtocolVersionReq,
    response::{CompareProtocolVersionRsp, IRsp},
};
use axum::extract::Json as JsonEx;
use axum::response::Json;

pub async fn post(
    JsonEx(_req): JsonEx<CompareProtocolVersionReq>,
) -> Json<IRsp<CompareProtocolVersionRsp>> {
    Json(IRsp::<CompareProtocolVersionRsp> {
        data: Some(CompareProtocolVersionRsp {
            modified: false,
            ..Default::default()
        }),
        ..Default::default()
    })
}
