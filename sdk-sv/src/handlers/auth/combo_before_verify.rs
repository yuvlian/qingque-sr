use crate::ArcState;
use crate::app::{
    request::ComboBeforeVerifyReq,
    response::{ComboBeforeVerifyRsp, IRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(_state): State<ArcState>,
    JsonEx(_req): JsonEx<ComboBeforeVerifyReq>,
) -> Json<IRsp<ComboBeforeVerifyRsp>> {
    todo!()
}
