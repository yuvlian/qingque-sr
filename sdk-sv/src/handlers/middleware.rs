use crate::ArcState;
use axum::{
    body::{Body, Bytes, to_bytes},
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{error, info};

pub async fn log_requests(
    State(log_level): State<i32>,
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let req_method = req.method().clone();
    let req_uri = req.uri().clone();
    let req_headers = req.headers().clone();
    let start_time = Instant::now();

    match log_level {
        1 => {
            let response = next.run(req).await;
            let res_status = response.status();
            let duration = start_time.elapsed();

            if res_status == StatusCode::INTERNAL_SERVER_ERROR {
                error!(
                    "{} {} - Status: {} | {:.2?}",
                    req_method, req_uri, res_status, duration
                );
            } else {
                info!(
                    "{} {} - Status: {} | {:.2?}",
                    req_method, req_uri, res_status, duration
                );
            }

            response
        }
        _ => {
            let req_body_bytes = to_bytes(req.into_body(), usize::MAX)
                .await
                .unwrap_or_default();

            let req_body = String::from_utf8_lossy(&req_body_bytes).into_owned();

            let mut req_builder = Request::builder()
                .method(req_method.clone())
                .uri(req_uri.clone());

            req_builder = req_headers
                .iter()
                .fold(req_builder, |b, (key, value)| b.header(key, value));

            let new_request = req_builder.body(Body::from(req_body_bytes)).unwrap();

            let response = next.run(new_request).await;
            let res_status = response.status();
            let res_headers = response.headers().clone();

            let res_body_bytes = to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_else(|_| {
                    error!("Failed to read response body");
                    Bytes::new()
                });

            let res_body = String::from_utf8_lossy(&res_body_bytes);
            let duration = start_time.elapsed();

            if res_status == StatusCode::INTERNAL_SERVER_ERROR {
                error!(
                    "\n{} {} - Status: {} | Duration: {:.2?}\n->> Request Headers: {:?}\n->> Request Body: {}\n->> Response Headers: {:?}\n->> Response Body: {}",
                    req_method,
                    req_uri,
                    res_status,
                    duration,
                    req_headers,
                    req_body,
                    res_headers,
                    res_body
                );
            } else {
                info!(
                    "\n{} {} - Status: {} | Duration: {:.2?}\n->> Request Headers: {:?}\n->> Request Body: {}\n->> Response Headers: {:?}\n->> Response Body: {}",
                    req_method,
                    req_uri,
                    res_status,
                    duration,
                    req_headers,
                    req_body,
                    res_headers,
                    res_body
                );
            }

            let mut res_builder = Response::builder().status(res_status);

            res_builder = res_headers
                .iter()
                .fold(res_builder, |b, (key, value)| b.header(key, value));

            res_builder.body(Body::from(res_body_bytes)).unwrap()
        }
    }
}
