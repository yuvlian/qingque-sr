use crate::app::{
    request::DeviceGetFpReq,
    response::{DeviceGetFpRsp, IRsp},
};
use axum::extract::Json as JsonEx;
use axum::response::Json;

pub async fn post(JsonEx(req): JsonEx<DeviceGetFpReq>) -> Json<IRsp<DeviceGetFpRsp>> {
    Json(IRsp::<DeviceGetFpRsp> {
        data: Some(DeviceGetFpRsp {
            device_fp: req.device_fp,
            code: 200,
            msg: "ok".to_string(),
        }),
        ..Default::default()
    })
}
