use axum::Json;
use serde_json::{Value, json};

pub async fn handle() -> Json<Value> {
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
