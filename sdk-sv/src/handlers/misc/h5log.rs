use crate::app::{
    request::{H5LogBodyReq, H5LogQueryReq},
    response::IRsp,
};
use axum::extract::{Json as JsonEx, Query};
use axum::response::Json;

pub async fn post(
    Query(_query): Query<H5LogQueryReq>,
    JsonEx(_body): JsonEx<H5LogBodyReq>,
) -> Json<IRsp<()>> {
    Json(IRsp::h5log_rsp())
}
