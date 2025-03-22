use crate::app::{
    request::GetAlertAnnReq,
    response::{GetAlertAnnRsp, IRsp},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_query): Query<GetAlertAnnReq>) -> Json<IRsp<GetAlertAnnRsp>> {
    Json(IRsp::<GetAlertAnnRsp> {
        data: Some(GetAlertAnnRsp {
            remind: true,
            extra_remind: true,
            ..Default::default()
        }),
        ..Default::default()
    })
}
