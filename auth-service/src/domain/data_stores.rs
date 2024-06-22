use super::{Email, LoginAttemptId, Password, TwoFACode, User};
use color_eyre::eyre::{Report, Result};
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
            | (Self::UserNotFound, Self::UserNotFound)
            | (Self::InvalidCredentials, Self::InvalidCredentials)
            | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> ;
}

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),

}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn store_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn check_banned_token(&self, token: String) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, Error)]
pub enum TwoFACodeStoreError {
    #[error("Login Attempt ID not found")]
    LoginAttemptIdNotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
            | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
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

