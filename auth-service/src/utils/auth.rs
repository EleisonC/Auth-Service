use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};

use crate::{app_state::BannedTokenStoreType, domain::{email::Email, AuthAPIError}};

use super::constants::{JWT_COOKIE_NAME, JWT_SECRET};

pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}


fn create_auth_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build();

    cookie
}

#[derive(Debug)]
pub enum GenerateTokenError {
    TokenError(jsonwebtoken::errors::Error),
    UnexpectedError,
}

pub const TOKEN_TTL_SECONDS: i64 = 600;

fn generate_auth_token(email: &Email) -> Result<String> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
            .wrap_err("failed to create 10 minute time delta")?;

    let exp = Utc::now()
        .checked_add_signed(delta)
        .ok_or(eyre!("failed to add 10 minutes to current time"))?
        .timestamp();

    let exp: usize = exp
        .try_into()
        .wrap_err(format!("failed to cast exp time to usize. exp time: {}", exp))?;

    let sub = email.as_ref().to_owned();

    let claims = Claims {sub, exp};

    create_token(&claims)
}

pub async fn validate_token(token: &str, banned_token_store: BannedTokenStoreType) -> Result<Claims> {

    let banned_tk_store = &banned_token_store.read().await;
    match banned_tk_store.check_banned_token(token.to_string()).await {
        Ok(value) => {
            if value {
                return Err(eyre!("token is banned"));
            }
        }  // Or another appropriate error
        Err(e) => return Err(e.into()),
    };

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .wrap_err("failed to decode token")
}

fn create_token(claims: &Claims) -> Result<String> {
    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .wrap_err("failed to create token")
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::services;

    use crate::domain::data_stores::BannedTokenStore;

    use super::*;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let cookie = generate_auth_cookie(&email).unwrap();
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_create_auth_cookie() {
        let token = "test_token".to_owned();
        let cookie = create_auth_cookie(token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_generate_auth_token() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let result = generate_auth_token(&email).unwrap();
        assert_eq!(result.split('.').count(), 3);
    }

    #[tokio::test]
    async fn test_validate_token_with_valid_token() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let token = generate_auth_token(&email).unwrap();
        let banned_token_store = Arc::new(RwLock::new(services::HashsetBannedTokenStore::default()));
        let result = validate_token(&token, banned_token_store).await.unwrap();
        assert_eq!(result.sub, "test@example.com");

        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
            .expect("valid timestamp")
            .timestamp();

        assert!(result.exp > exp as usize);
    }

    #[tokio::test]
    async fn test_validate_token_with_invalid_token() {
        let token = "invalid_token".to_owned();
        let banned_token_store = Arc::new(RwLock::new(services::HashsetBannedTokenStore::default()));
        let result = validate_token(&token, banned_token_store).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_with_banned_token() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let token = generate_auth_token(&email).unwrap();
        let banned_token_store = Arc::new(RwLock::new(services::HashsetBannedTokenStore::default()));

        {
            let mut banned_tk_store = banned_token_store.write().await;
            banned_tk_store.store_banned_token(token.clone()).await.unwrap();
        }

        let result = validate_token(&token, banned_token_store).await.is_err();

        assert_eq!(result, true);
    }
}
