use crate::ArcState;
use crate::app::{
    request::ComboBeforeVerifyReq,
    response::{ComboBeforeVerifyRsp, IRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;
use db::sdk::user::User;

pub async fn post(
    State(state): State<ArcState>,
    JsonEx(req): JsonEx<ComboBeforeVerifyReq>,
) -> Json<IRsp<ComboBeforeVerifyRsp>> {
    let combo_token = req.combo_token;
    let uid = match req.role.uid.parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            return Json(IRsp::<ComboBeforeVerifyRsp>::custom_error(
                -101,
                "Invalid UID".to_string(),
            ));
        }
    };

    let rsp = match User::verify_combo_token_by_uid(&state.pool, uid, &combo_token).await {
        Ok(true) => IRsp::<ComboBeforeVerifyRsp>::ok(ComboBeforeVerifyRsp::default()),
        Ok(false) => {
            IRsp::<ComboBeforeVerifyRsp>::custom_error(-101, "Combo Token Mismatch".to_string())
        }
        Err(_) => IRsp::<ComboBeforeVerifyRsp>::internal_error(),
    };

    Json(rsp)
}
