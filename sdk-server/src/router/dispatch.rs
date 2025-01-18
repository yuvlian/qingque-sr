use crate::handlers::dispatch::{query_dispatch, query_gateway};
use axum::{Router, routing::get};

pub fn dispatch_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route("/query_dispatch", get(query_dispatch::handle));

    router = router.route("/query_gateway", get(query_gateway::handle));

    router
}
