use axum::Json;
use serde_json::{Value, json};

pub async fn handle() -> Json<Value> {
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
                "account_name": "yulian",
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
