use std::collections::HashMap;
use crate::domain::{User, UserStore, UserStoreError};



#[derive(Default, Debug)]
pub struct HashmapUserStore {
    users: HashMap<String, User>
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let current_user_email = user.email.clone();

        if self.users.contains_key(&current_user_email) {
            return Err(UserStoreError::UserAlreadyExists)
        }

        self.users.insert(current_user_email, user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            return Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            if user.password == password {
                return Ok(())
            }
            return Err(UserStoreError::InvalidCredentials)
        } else {
            return Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "user.test@mail.com".to_string(),
            "password".to_string(),
            false
        );

        let result = store.add_user(user.clone()).await;
        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "user.test@mail.com".to_string(),
            "password".to_string(),
            false
        );
        store.add_user(user.clone()).await.unwrap();

        let result = store.get_user(&user.email).await;
        match result {
            Ok(u) => assert_eq!(u, user),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "user.test@mail.com".to_string(),
            "password".to_string(),
            true
        );

        store.add_user(user.clone()).await.unwrap();

        let result = store.validate_user(&user.email, &user.password).await;
        assert_eq!(result, Ok(()));
    }
}


