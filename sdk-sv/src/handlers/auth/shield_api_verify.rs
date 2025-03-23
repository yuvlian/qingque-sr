use crate::ArcState;
use crate::app::{
    request::ShieldApiVerifyReq,
    response::{IRsp, ShieldAccount, ShieldApiVerifyRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;
use db::sdk::user::User;

pub async fn post(
    State(state): State<ArcState>,
    JsonEx(req): JsonEx<ShieldApiVerifyReq>,
) -> Json<IRsp<ShieldApiVerifyRsp>> {
    let token = req.token;
    let uint_uid = match req.uid.parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            return Json(IRsp::<ShieldApiVerifyRsp>::custom_error(
                -101,
                "Invalid UID".to_string(),
            ));
        }
    };
    let uid = req.uid;

    let rsp = match User::verify_token_by_uid(&state.pool, uint_uid, &token).await {
        Ok(true) => IRsp::<ShieldApiVerifyRsp>::ok(ShieldApiVerifyRsp {
            realname_operation: "None".to_string(),
            account: ShieldAccount {
                uid,
                token,
                name: "stalel".to_string(),
                email: "stalel@neonteam.dev".to_string(),
                mobile: "1".to_string(),
                is_email_verify: "0".to_string(),
                country: "ID".to_string(),
                area_code: "**".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }),
        Ok(false) => IRsp::<ShieldApiVerifyRsp>::custom_error(-101, "Token Mismatch".to_string()),
        Err(_) => IRsp::<ShieldApiVerifyRsp>::internal_error(),
    };

    Json(rsp)
}
