pub mod risky {
    pub mod api {
        use axum::{http::StatusCode, Json};
        use serde_json::{json, Value};

        pub async fn check() -> (StatusCode, Json<Value>) {
            let rsp = Json(json!({
                "data": {},
                "message": "OK",
                "retcode": 0
            }));

            (StatusCode::OK, rsp)
        }
    }
}
