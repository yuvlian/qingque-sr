use axum::Json;
use serde_json::{Value, json};

pub async fn handle() -> Json<Value> {
    Json(json!({
        "data": {},
        "message": "OK",
        "retcode": 0
    }))
}
