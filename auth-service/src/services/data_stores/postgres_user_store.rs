use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2,
    Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    } 
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
            user.email.as_ref(),
            user.password.as_ref(),
            user.requires2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_|UserStoreError::UnexpectedError);

        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        let user_row: User = sqlx::query_as(
            r#"
            SELECT email as email, password_hash as password, requires_2fa as requires2fa FROM users WHERE email = $1
            "#
        ).bind(email.as_ref()).fetch_one(&self.pool).await.map_err(|_| UserStoreError::UserNotFound)?;

        Ok(user_row)
    }

    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {

    }
}

fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), Box<dyn Error>> {
    let expected_password_hash = PasswordHash::new(expected_password_hash)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .map_err(|e| e.into())
}

fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
    
}
