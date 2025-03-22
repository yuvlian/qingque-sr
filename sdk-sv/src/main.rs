mod app;
mod handlers;

use axum::{Router, middleware::from_fn_with_state};
use handlers::middleware;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::net::TcpListener;
use utils::{DotEnv, init_tracing};

struct AppState {
    env: Arc<DotEnv>,
    pool: Arc<SqlitePool>,
}

type ArcState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let env = Arc::new(DotEnv::load());
    let log_level = env.log_level;
    let pool = Arc::new(SqlitePool::connect(&env.database_url).await.unwrap());

    let state = Arc::new(AppState {
        env: env.clone(),
        pool,
    });

    let addr = format!("{}:{}", env.sdk_sv_host, env.sdk_sv_port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    let app = Router::new()
        .merge(app::router::auth_router())
        .merge(app::router::dispatch_router())
        .merge(app::router::misc_router())
        .with_state(state.clone());

    match log_level {
        1 | 2 => {
            init_tracing();
            tracing::info!("Listening @ {}", addr);

            let app = app.layer(from_fn_with_state(log_level, middleware::log_requests));
            axum::serve(listener, app).await.unwrap();
        }
        _ => {
            axum::serve(listener, app).await.unwrap();
        }
    }
}
