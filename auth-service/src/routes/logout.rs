use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token,
        constants::JWT_COOKIE_NAME
    },
};


pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match  jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        _ => return (jar, Err(AuthAPIError::MissingToken))
    };

    let token = cookie.value().to_owned();

    if validate_token(&token).await.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken))
    }

    let jar = jar.remove(JWT_COOKIE_NAME);
    
    (jar, Ok(StatusCode::OK))
}
