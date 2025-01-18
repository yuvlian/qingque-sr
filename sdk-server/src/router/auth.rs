use crate::handlers::auth::{account, product};
use axum::{Router, routing::post};

pub fn auth_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route(
        "/account/risky/api/check",
        post(account::risky::api::check::handle),
    );

    router = router.route(
        "/:product/mdk/shield/api/login",
        post(product::mdk::shield::api::login::handle),
    );

    router = router.route(
        "/:product/mdk/shield/api/verify",
        post(product::mdk::shield::api::verify::handle),
    );

    router = router.route(
        "/:product/combo/granter/login/v2/login",
        post(product::combo::granter::login::v2::login::handle),
    );

    router
}
