use std::collections::HashSet;

use secrecy::{ExposeSecret, Secret};

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default, Debug)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_banned_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token.expose_secret().to_owned());
        Ok(())
    }

    async fn check_banned_token(&self, token: Secret<String>) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_banned_token() {
        let mut store = HashsetBannedTokenStore::default();

        let test_token = Secret::new("thisewweeqeqweqwe321321343424324=-w".to_string());

        let result = store.store_banned_token(test_token).await.unwrap();

        assert_eq!(result, ())
    }

    #[tokio::test]
    async fn test_check_banned_token_valid() {
        let mut store = HashsetBannedTokenStore::default();

        let test_token = Secret::new("thisewweeqeqweqwe321321343424324=-w".to_string());

        store.store_banned_token(test_token.clone()).await.unwrap();

        let result = store.check_banned_token(test_token.clone()).await.unwrap();

        assert_eq!(result, true)
    }

    #[tokio::test]
    async fn test_check_banned_token_invalid() {
        let store = HashsetBannedTokenStore::default();

        let test_token = Secret::new("thisewweeqeqweqwe321321343424324=-w".to_string());

        let result = store.check_banned_token(test_token.clone()).await.unwrap();

        assert_eq!(result, false)
    }
}

