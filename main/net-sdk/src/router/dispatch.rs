use crate::handlers::dispatch;
use axum::{routing::get, Router};

pub fn dispatch_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route("/query_dispatch", get(dispatch::query_dispatch));

    router = router.route("/query_gateway", get(dispatch::query_gateway));

    router
}
