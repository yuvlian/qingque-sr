use crate::app::{
    request::SwPrecacheReq,
    response::{IRsp, SwPrecacheRsp, SwPrecacheVals},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_query): Query<SwPrecacheReq>) -> Json<IRsp<SwPrecacheRsp>> {
    Json(IRsp::<SwPrecacheRsp> {
        data: Some(SwPrecacheRsp {
            vals: SwPrecacheVals {
                url: "https://proxy1.neonteam.dev/sdk.hoyoverse.com/sw.html".to_string(),
                enable: "true".to_string(),
            },
        }),
        ..Default::default()
    })
}
