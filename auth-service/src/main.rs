use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::RwLock;

use auth_service::{app_state::AppState, get_postgres_pool, services, utils::constants::{prod, DATABASE_URL}, Application};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(services::HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(services::HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(services::HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(services::MockEmailClient::default()));

    let pg_pool = configure_postgresql().await;

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
