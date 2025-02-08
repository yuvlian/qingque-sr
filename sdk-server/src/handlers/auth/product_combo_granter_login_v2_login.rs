use axum::Json;
use serde_json::{Value, json};

pub async fn handle() -> Json<Value> {
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
