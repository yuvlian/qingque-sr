pub mod mdk {
    pub mod shield {
        pub mod api {
            use axum::{http::StatusCode, Json};
            use serde_json::{json, Value};

            pub async fn login() -> (StatusCode, Json<Value>) {
                let rsp = Json(json!({
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
                }));

                (StatusCode::OK, rsp)
            }

            pub async fn verify() -> (StatusCode, Json<Value>) {
                let rsp = Json(json!({
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
                }));

                (StatusCode::OK, rsp)
            }
        }
    }
}

pub mod combo {
    pub mod granter {
        pub mod login {
            pub mod v2 {
                use axum::{http::StatusCode, Json};
                use serde_json::{json, Value};

                pub async fn login() -> (StatusCode, Json<Value>) {
                    let rsp = Json(json!({
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
                    }));

                    (StatusCode::OK, rsp)
                }
            }
        }
    }
}
