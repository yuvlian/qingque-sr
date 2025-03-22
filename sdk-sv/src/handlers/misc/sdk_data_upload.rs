use crate::app::{
    request::SdkDataUploadReq,
    response::{IRsp, SdkDataUploadRsp},
};
use axum::extract::Json as JsonEx;
use axum::response::Json;

pub async fn post(JsonEx(_req): JsonEx<SdkDataUploadReq>) -> Json<IRsp<SdkDataUploadRsp>> {
    Json(IRsp::<SdkDataUploadRsp> {
        data: Some(SdkDataUploadRsp::default()),
        ..Default::default()
    })
}
