use crate::app::{
    request::GetAlertPicReq,
    response::{GetAlertPicRsp, IRsp},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_query): Query<GetAlertPicReq>) -> Json<IRsp<GetAlertPicRsp>> {
    Json(IRsp::<GetAlertPicRsp> {
        data: Some(GetAlertPicRsp::default()),
        ..Default::default()
    })
}
