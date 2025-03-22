use crate::ArcState;
use crate::app::{
    request::ShieldApiVerifyReq,
    response::{IRsp, ShieldApiVerifyRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(_state): State<ArcState>,
    JsonEx(_req): JsonEx<ShieldApiVerifyReq>,
) -> Json<IRsp<ShieldApiVerifyRsp>> {
    todo!()
}
