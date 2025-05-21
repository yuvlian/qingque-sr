use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let response = next.run(req).await;
    let status = response.status();

    info!("{} - {} {}", status, method, uri);

    response
}
