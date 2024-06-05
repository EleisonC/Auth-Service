use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{app_state::AppState, services, Application, utils::constants::prod};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(services::HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(services::HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(services::HashmapTwoFACodeStore::default()));
    let app_state = AppState::new(
        user_store, banned_token_store, two_fa_code_store);
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
