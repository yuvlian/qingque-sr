use crate::app::{
    request::GetAgreementInfosReq,
    response::{GetAgreementInfosRsp, IRsp},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_query): Query<GetAgreementInfosReq>) -> Json<IRsp<GetAgreementInfosRsp>> {
    Json(IRsp::<GetAgreementInfosRsp> {
        data: Some(GetAgreementInfosRsp::default()),
        ..Default::default()
    })
}
