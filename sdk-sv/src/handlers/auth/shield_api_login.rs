use crate::AppState;
use crate::app::{
    request::ShieldApiLoginReq,
    response::{IRsp, ShieldApiLoginRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(state): State<AppState>,
    JsonEx(req): JsonEx<ShieldApiLoginReq>,
) -> Json<IRsp<ShieldApiLoginRsp>> {
    todo!()
}
