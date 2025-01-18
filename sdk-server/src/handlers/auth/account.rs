pub mod risky {
    pub mod api {
        pub mod check {
            use axum::{Json, http::StatusCode};
            use serde_json::{Value, json};

            pub async fn handle() -> (StatusCode, Json<Value>) {
                let rsp = Json(json!({
                    "data": {},
                    "message": "OK",
                    "retcode": 0
                }));

                (StatusCode::OK, rsp)
            }
        }
    }
}
