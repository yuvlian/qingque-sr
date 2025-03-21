use crate::AppState;
use crate::app::{
    request::ComboGranterLoginReq,
    response::{ComboGranterLoginRsp, IRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(state): State<AppState>,
    JsonEx(req): JsonEx<ComboGranterLoginReq>,
) -> Json<IRsp<ComboGranterLoginRsp>> {
    todo!()
}
