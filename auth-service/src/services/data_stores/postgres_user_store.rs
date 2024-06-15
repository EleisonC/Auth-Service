use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2,
    Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version
};

use sqlx::PgPool;
use tokio::task::spawn_blocking;

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
        let pass = user.password.clone();
        let password_hash = spawn_blocking(move || compute_password_hash(pass.as_ref()))
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;
        // let user_hash = password_hash.map_err(|_| Err(UserStoreError::UnexpectedError))?;
        let user_hash;
        if let Ok(hash) = password_hash {
            user_hash = hash;
        } else {
            return Err(UserStoreError::UnexpectedError)
        };

        sqlx::query(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(user.email.as_ref())
        .bind(user_hash)
        .bind(user.requires2fa)
        .execute(&self.pool)
        .await
        .map_err(|_|UserStoreError::UserAlreadyExists)?;

        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        let user_row: User = sqlx::query_as(
            r#"
            SELECT email as email, password_hash, requires_2fa as requires2fa
            FROM users 
            WHERE email = $1
            "#
        ).bind(email.as_ref()).fetch_one(&self.pool).await.map_err(|_| UserStoreError::UserNotFound)?;

        Ok(user_row)
    }

    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
        let valid_user: User = sqlx::query_as(
            r#"
            SELECT email, password_hash, requires_2fa as requires2fa
            FROM users
            WHERE email = $1
            "#
        ).bind(email.as_ref()).fetch_one(&self.pool).await.map_err(|e| {
            eprintln!("Error fetching user from database: {:?}", e);
            UserStoreError::UserNotFound
        })?;

        if verify_password_hash(valid_user.password.as_ref(), password.as_ref()).is_err() {
            return Err(UserStoreError::InvalidCredentials)
        }

        Ok(())
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

fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

    Ok(password_hash)
}
