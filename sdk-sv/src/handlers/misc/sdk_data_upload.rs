use crate::app::response::{IRsp, SdkDataUploadRsp};
// use crate::app::request::SdkDataUploadReq;
// use axum::extract::Json as JsonEx;
use axum::response::Json;

// status code 522 so we'll just remove the extractor cuz im not gonna bother fixing the model
// pub async fn post(JsonEx(_req): JsonEx<SdkDataUploadReq>) -> Json<IRsp<SdkDataUploadRsp>> {
pub async fn post() -> Json<IRsp<SdkDataUploadRsp>> {
    Json(IRsp::<SdkDataUploadRsp> {
        data: Some(SdkDataUploadRsp::default()),
        ..Default::default()
    })
}
