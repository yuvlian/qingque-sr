use axum::{Json, Router, routing::post};
use serde_json::{Value, json};

pub fn auth_router() -> Router<()> {
    let mut router = Router::new();
    router = router.route(ON_RISKY_API_ROUTE, post(on_risky_api));
    router = router.route(ON_MDK_SHIELD_LOGIN_ROUTE, post(on_mdk_shield));
    router = router.route(ON_MDK_SHIELD_VERIFY_ROUTE, post(on_mdk_shield));
    router = router.route(ON_COMBO_GRANTER_LOGIN_ROUTE, post(on_combo_granter_login));
    router = router.route(ON_MA_PASSPORT_LOGIN_ROUTE, post(on_ma_passport_login));
    router
}

const ON_RISKY_API_ROUTE: &str = "/account/risky/api/check";
async fn on_risky_api() -> Json<Value> {
    Json(json!({
        "data": {},
        "message": "OK",
        "retcode": 0
    }))
}

const ON_MDK_SHIELD_LOGIN_ROUTE: &str = "/{product}/mdk/shield/api/login";
const ON_MDK_SHIELD_VERIFY_ROUTE: &str = "/{product}/mdk/shield/api/verify";
async fn on_mdk_shield() -> Json<Value> {
    Json(json!({
        "data": {
            "account": {
                "area_code": "**",
                "email": "yuvlian@naver.com",
                "country": "ID",
                "is_email_verify": "1",
                "token": "x",
                "uid": "1"
            },
            "device_grant_required": false,
            "reactivate_required": false,
            "realperson_required": false,
            "safe_mobile_required": false
        },
        "message": "OK",
        "retcode": 0
    }))
}

const ON_COMBO_GRANTER_LOGIN_ROUTE: &str = "/{product}/combo/granter/login/v2/login";
async fn on_combo_granter_login() -> Json<Value> {
    Json(json!({
        "data": {
            "account_type": 1,
            "combo_id": "1",
            "combo_token": "x",
            "data": "{\"guest\":false}",
            "heartbeat": false,
            "open_id": "1"
        },
        "message": "OK",
        "retcode": 0
    }))
}

const ON_MA_PASSPORT_LOGIN_ROUTE: &str = "/{product}/account/ma-passport/api/appLoginByPassword";
async fn on_ma_passport_login() -> Json<Value> {
    Json(json!({
        "data": {
            "bind_email_action_ticket": "",
            "ext_user_info": {
                "birth": "0",
                "guardian_email": ""
            },
            "reactivate_action_token": "",
            "token": {
                "token_type": 1,
                "token": "x"
            },
            "user_info": {
                "account_name": "yuvlian",
                "aid": "1",
                "area_code": "**",
                "token": "x",
                "email": "yuvlian@naver.com",
                "is_email_verify": "1",
                "country": "ID"
            }
        },
        "message": "OK",
        "retcode": 0
    }))
}
