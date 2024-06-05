use super::{Email, LoginAttemptId, Password, TwoFACode, User};

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> ;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    UnexpectedError,
    TokenNotFound,
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn store_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn check_banned_token(&self, token: String) -> Result<String, BannedTokenStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(&mut self, 
        email: Email, 
        login_attempt_id: LoginAttemptId,
        code: TwoFACode
    ) -> Result<(), TwoFACodeStoreError>;

    async fn remove_code(&mut self, 
        email: &Email
    ) -> Result<(), TwoFACodeStoreError>;

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

