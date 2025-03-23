use crate::ArcState;
use crate::app::{
    request::ShieldApiLoginReq,
    response::{IRsp, ShieldAccount, ShieldApiLoginRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;
use db::sdk::user::User;

pub async fn post(
    State(state): State<ArcState>,
    JsonEx(req): JsonEx<ShieldApiLoginReq>,
) -> Json<IRsp<ShieldApiLoginRsp>> {
    let (username, password) = {
        // we're doing this cuz i'm not gonna bother RSA patching
        let kv: Vec<&str> = req.account.split("::").collect();

        if kv.len() != 2 {
            return Json(IRsp::<ShieldApiLoginRsp>::custom_error(
                -101,
                "Invalid {user}::{pass} Input".to_string(),
            ));
        }

        (kv[0], kv[1])
    };

    let rsp = match User::verify_login_form_and_give_token(&state.pool, username, password).await {
        Ok(v) => IRsp::<ShieldApiLoginRsp>::ok(ShieldApiLoginRsp {
            realname_operation: "None".to_string(),
            account: ShieldAccount {
                uid: v.0.to_string(),
                token: v.1,
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
        Err(e) => IRsp::<ShieldApiLoginRsp>::custom_error(-101, e.to_string()),
    };

    Json(rsp)
}
