mod app;
mod handlers;

use axum::{Router, extract::State, middleware::from_fn_with_state};
use handlers::middleware;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::net::TcpListener;
use utils::{DotEnv, init_tracing};

#[derive(Clone)]
pub struct AppState {
    pub env: Arc<DotEnv>,
    pub pool: Arc<SqlitePool>,
}

#[tokio::main]
async fn main() {
    let env = Arc::new(DotEnv::load());
    let pool = Arc::new(SqlitePool::connect(&env.database_url).await.unwrap());

    let state = AppState {
        env: env.clone(),
        pool,
    };

    let addr = format!("{}:{}", env.sdk_sv_host, env.sdk_sv_port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    init_tracing();

    tracing::info!("Listening @ {}", addr);

    let app = Router::new()
        .merge(app::router::auth_router())
        .with_state(state.clone());

    if env.log_level == 1 || env.log_level == 2 {
        let app = app.layer(from_fn_with_state(
            state.env.log_level,
            middleware::log_requests,
        ));
        axum::serve(listener, app).await.unwrap();
    } else {
        axum::serve(listener, app).await.unwrap();
    }
}
