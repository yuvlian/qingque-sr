use crate::ArcState;
use crate::app::{
    request::ShieldApiVerifyReq,
    response::{IRsp, ShieldApiVerifyRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(state): State<ArcState>,
    JsonEx(req): JsonEx<ShieldApiVerifyReq>,
) -> Json<IRsp<ShieldApiVerifyRsp>> {
    todo!()
}
