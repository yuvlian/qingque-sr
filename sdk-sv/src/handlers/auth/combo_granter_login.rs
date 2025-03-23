use crate::ArcState;
use crate::app::{
    request::ComboGranterLoginReq,
    response::{ComboGranterLoginRsp, IRsp},
};
use axum::extract::Json as JsonEx;
use axum::extract::State;
use axum::response::Json;
use db::sdk::user::User;

pub async fn post(
    State(state): State<ArcState>,
    JsonEx(req): JsonEx<ComboGranterLoginReq>,
) -> Json<IRsp<ComboGranterLoginRsp>> {
    let data = match req.parse_data() {
        Ok(v) => v,
        Err(_) => {
            return Json(IRsp::<ComboGranterLoginRsp>::custom_error(
                -101,
                "Invalid Data".to_string(),
            ));
        }
    };

    let rsp = match User::verify_token_and_give_combo_token(
        &state.pool,
        &data.username,
        &data.token,
        &req.device,
    )
    .await
    {
        Ok(v) => IRsp::<ComboGranterLoginRsp>::ok(ComboGranterLoginRsp {
            combo_id: data.uid.clone(),
            open_id: data.uid,
            combo_token: v,
            data: r#"{"guest":false}"#.to_string(),
            account_type: 1,
            ..Default::default()
        }),
        Err(e) => IRsp::<ComboGranterLoginRsp>::custom_error(-101, e.to_string()),
    };

    Json(rsp)
}
