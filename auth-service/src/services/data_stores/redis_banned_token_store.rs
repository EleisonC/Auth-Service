use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{
        BannedTokenStore, BannedTokenStoreError},
        utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self{ conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn store_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let redis_token_key = get_key(&token);
        let mut store_conn = self.conn.write().await;

        let ttl = TOKEN_TTL_SECONDS as u64;
        let result = store_conn.set_ex::<String, bool, ()>(redis_token_key, true, ttl);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(BannedTokenStoreError::UnexpectedError)
        }
    }

    async fn check_banned_token(&self, token: String) -> Result<String, BannedTokenStoreError> {
        let redis_token_key = get_key(&token);
        let mut store_conn = self.conn.write().await;

        let result = store_conn.exists(redis_token_key);

        match result {
            Ok(true) => Ok(format!("Token {} is banned", token)),
            Ok(false) => Err(BannedTokenStoreError::TokenNotFound),
            Err(_) => Err(BannedTokenStoreError::UnexpectedError)
        }
    }
}

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    return format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token);
}
