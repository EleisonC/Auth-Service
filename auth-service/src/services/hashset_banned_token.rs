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

    async fn check_banned_token(&self, token: String) -> Result<String, BannedTokenStoreError> {
        if !self.banned_tokens.contains(&token) {
            return Err(BannedTokenStoreError::TokenNotFound)
        }
        Ok(format!("Token {} is banned", token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_banned_token() {
        let mut store = HashsetBannedTokenStore::default();

        let test_token = "thisewweeqeqweqwe321321343424324=-w".to_string();

        let result = store.store_banned_token(test_token).await;

        assert_eq!(result, Ok(())) 
    }

    #[tokio::test]
    async fn test_check_banned_token_valid() {
        let mut store = HashsetBannedTokenStore::default();

        let test_token = "thisewweeqeqweqwe321321343424324=-w".to_string();

        store.store_banned_token(test_token.clone()).await.unwrap();

        let result = store.check_banned_token(test_token.clone()).await;

        assert_eq!(result, Ok(format!("Token {} is banned", test_token))) 
    }

    #[tokio::test]
    async fn test_check_banned_token_invalid() {
        let store = HashsetBannedTokenStore::default();

        let test_token = "thisewweeqeqweqwe321321343424324=-w".to_string();

        let result = store.check_banned_token(test_token.clone()).await;

        assert_eq!(result, Err(BannedTokenStoreError::TokenNotFound)) 
    }
}

