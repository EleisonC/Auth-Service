use std::collections::HashMap;

use crate::domain::{
    data_stores::{TwoFACodeStore, TwoFACodeStoreError},
    loginattemptid::LoginAttemptId,
    twofacode::TwoFACode,
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore{
    async fn add_code(&mut self, 
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        if self.codes.remove(email).is_some() {
            Ok(())
        } else {
            return Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        if let Some(data) = self.codes.get(email) {
            Ok((data.0.clone(), data.1.clone()))
        } else {
            return Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("user.test@mail.com".to_string()).unwrap();
        let login_attempt = LoginAttemptId::parse(LoginAttemptId::default().as_ref().to_string()).unwrap();
        let code = TwoFACode::parse(TwoFACode::default().as_ref().to_string()).unwrap();

        let result = store.add_code(email, login_attempt, code).await;

        assert_eq!(result, Ok(()))
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("user.test@mail.com".to_string()).unwrap();
        let login_attempt = LoginAttemptId::parse(LoginAttemptId::default().as_ref().to_string()).unwrap();
        let code = TwoFACode::parse(TwoFACode::default().as_ref().to_string()).unwrap();

        store.add_code(email.clone(), login_attempt, code).await.unwrap();

        let result = store.remove_code(&email).await;

        assert_eq!(result, Ok(()))
    }

    #[tokio::test]
    async fn test_remove_code_not_found() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("user.test@mail.com".to_string()).unwrap();

        let result = store.remove_code(&email).await;

        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound))
    }
}
