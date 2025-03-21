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

    r
}
