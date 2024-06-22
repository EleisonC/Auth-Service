use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default, Debug)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token);
        Ok(())
    }

    async fn check_banned_token(&self, token: String) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(&token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_banned_token() {
        let mut store = HashsetBannedTokenStore::default();

        let test_token = "thisewweeqeqweqwe321321343424324=-w".to_string();

        let result = store.store_banned_token(test_token).await.unwrap();

        assert_eq!(result, ())
    }

    #[tokio::test]
    async fn test_check_banned_token_valid() {
        let mut store = HashsetBannedTokenStore::default();

        let test_token = "thisewweeqeqweqwe321321343424324=-w".to_string();

        store.store_banned_token(test_token.clone()).await.unwrap();

        let result = store.check_banned_token(test_token.clone()).await.unwrap();

        assert_eq!(result, true)
    }

    #[tokio::test]
    async fn test_check_banned_token_invalid() {
        let store = HashsetBannedTokenStore::default();

        let test_token = "thisewweeqeqweqwe321321343424324=-w".to_string();

        let result = store.check_banned_token(test_token.clone()).await.unwrap();

        assert_eq!(result, false)
    }
}

