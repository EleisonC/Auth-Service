use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;
use color_eyre::eyre::Context;

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
    #[tracing::instrument(name= "Store a banned token to Redis", skip_all)]
    async fn store_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let redis_token_key = get_key(&token);
        let mut store_conn = self.conn.write().await;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("failed to cast TOKEN_TTL_SECONDS u64")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        let _: () = store_conn
            .set_ex(redis_token_key, true, ttl)
            .wrap_err("failed to set banned token in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(())

    }

    #[tracing::instrument(name= "Check for banned token in Redis", skip_all)]
    async fn check_banned_token(&self, token: String) -> Result<bool, BannedTokenStoreError> {
        let redis_token_key = get_key(&token);
        let mut store_conn = self.conn.write().await;

        let result: bool = store_conn
            .exists(redis_token_key)
            .wrap_err("failed to check if token exists in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

       Ok(result)
    }
}

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    return format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token);
}
