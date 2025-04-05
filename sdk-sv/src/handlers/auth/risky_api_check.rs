use crate::ArcState;
use crate::app::{
    request::RiskyApiCheckReq,
    response::{IRsp, RiskyApiCheckRsp},
};
use axum::extract::{Json as JsonEx, State};
use axum::http::header::HeaderMap;
use axum::response::Json;
use db::sdk::user::User;

pub async fn post(
    State(state): State<ArcState>,
    headers: HeaderMap,
    JsonEx(req): JsonEx<RiskyApiCheckReq>,
) -> Json<IRsp<RiskyApiCheckRsp>> {
    if req.action_type != "login" {
        return Json(IRsp::<RiskyApiCheckRsp>::ok(RiskyApiCheckRsp {
            id: "".to_string(),
            action: "ACTION_NONE".to_string(),
            ..Default::default()
        }));
    }

    let (username, _) = {
        // we're doing this cuz i'm not gonna bother RSA patching
        let kv: Vec<&str> = req.username.split("::").collect();

        if kv.len() < 2 {
            return Json(IRsp::<RiskyApiCheckRsp>::custom_error(
                -101,
                "Invalid {user}::{pass} Input".to_string(),
            ));
        }

        (kv[0], kv[1])
    };

    let device_id = match headers.get("x-rpc-device_id") {
        Some(v) => match v.to_str() {
            Ok(id) => id,
            _ => return Json(IRsp::<RiskyApiCheckRsp>::internal_error()),
        },
        _ => {
            return Json(IRsp::<RiskyApiCheckRsp>::custom_error(
                -101,
                "DeviceId Header is missing".to_string(),
            ));
        }
    };

    match User::insert_device_id_by_username(&state.pool, username, &device_id).await {
        Ok(_) => Json(IRsp::<RiskyApiCheckRsp>::ok(RiskyApiCheckRsp {
            id: "".to_string(),
            action: "ACTION_NONE".to_string(),
            ..Default::default()
        })),
        Err(_) => Json(IRsp::<RiskyApiCheckRsp>::internal_error()),
    }
}
