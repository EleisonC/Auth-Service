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
    #[tracing::instrument(name= "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let pass = user.password.clone();

        let password_hash = compute_password_hash(pass.as_ref().to_string()).await.map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#, user.email.as_ref(), password_hash, user.requires2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_|UserStoreError::UnexpectedError)?;

        Ok(())
    }

    #[tracing::instrument(name= "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        let user_row = sqlx::query_as!(User,
            r#"
            SELECT email as email, password_hash as password, requires_2fa as requires2fa
            FROM users 
            WHERE email = $1
            "#,
            email.as_ref()
        ).fetch_one(&self.pool).await.map_err(|_| UserStoreError::UserNotFound)?;

        Ok(user_row)
    }

    #[tracing::instrument(name= "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
        let valid_user: User = sqlx::query_as!(
            User,
            r#"
            SELECT email, password_hash as password, requires_2fa as requires2fa
            FROM users
            WHERE email = $1
            "#,
            email.as_ref()
        ).fetch_one(&self.pool).await.map_err(|e| {
            eprintln!("Error fetching user from database: {:?}", e);
            UserStoreError::UserNotFound
        })?;

        if verify_password_hash(valid_user.password.as_ref().to_string(), password.as_ref().to_string()).await.is_err() {
            return Err(UserStoreError::InvalidCredentials)
        }

        Ok(())
    }
}

#[tracing::instrument(name= "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let current_span: tracing::Span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {

        current_span.in_scope(|| {
            let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

            Argon2::default()
                .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                .map_err(|e| e.into())
        })
        
    })
    .await;

    result?
    
}

#[tracing::instrument(name= "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let current_span: tracing::Span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt = SaltString::generate(&mut rand::thread_rng());

            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?
            )
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

            Ok(password_hash)
        })
    }).await;

    result?
    
}
