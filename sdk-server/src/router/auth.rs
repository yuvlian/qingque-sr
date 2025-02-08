use crate::handlers::auth::*;
use axum::{Router, routing::post};

pub fn auth_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route(
        "/account/risky/api/check",
        post(account_risky_api_check::handle),
    );

    router = router.route(
        "/{product}/mdk/shield/api/login",
        post(product_mdk_shield_api_login::handle),
    );

    router = router.route(
        "/{product}/mdk/shield/api/verify",
        post(product_mdk_shield_api_verify::handle),
    );

    router = router.route(
        "/{product}/combo/granter/login/v2/login",
        post(product_combo_granter_login_v2_login::handle),
    );

    router = router.route(
        "/{product}/account/ma-passport/api/appLoginByPassword",
        post(product_account_ma_passport_api_app_login_by_password::handle),
    );

    router
}
