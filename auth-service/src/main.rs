use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::RwLock;

use auth_service::{app_state::AppState, get_postgres_pool, get_redis_client, services, utils::constants::{prod, DATABASE_URL, REDIS_HOST_NAME}, Application};

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    let redis_client = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(services::PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(services::RedisBannedTokenStore::new(redis_client)));
    let two_fa_code_store = Arc::new(RwLock::new(services::HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(services::MockEmailClient::default()));

    let app_state = AppState::new(
        user_store, banned_token_store, two_fa_code_store, email_client);
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");
    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis client")
}
