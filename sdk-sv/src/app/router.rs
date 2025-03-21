use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{auth, dispatch, misc};

pub fn auth_router() -> Router<AppState> {
    let mut r = Router::new();

    r = r.route("/my_register", get(auth::my_register::get));

    r = r.route("/my_register", post(auth::my_register::post));

    r = r.route(
        "/account/risky/api/check",
        post(auth::risky_api_check::post),
    );

    r = r.route(
        "/{product}/mdk/shield/api/login",
        post(auth::shield_api_login::post),
    );

    r = r.route(
        "/{product}/mdk/shield/api/verify",
        post(auth::shield_api_verify::post),
    );

    r = r.route(
        "/{product}/combo/granter/login/v2/login",
        post(auth::combo_granter_login::post),
    );

    r
}
