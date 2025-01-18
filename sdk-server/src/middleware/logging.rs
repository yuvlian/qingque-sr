use axum::http::StatusCode;
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::info;

pub async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    let status = response.status();

    if status == StatusCode::OK {
        info!("{} {} -> {} {:.2?}", method, uri, status, duration);
    }

    response
}
