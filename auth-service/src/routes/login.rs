use axum::{http::StatusCode, extract::State,
    response::IntoResponse, Json};
use serde::Deserialize;
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}


pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials))
    };

    let password = match Password::parse(request.password.clone()) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials))
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(email.clone(), password.clone()).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials))
    }

    if user_store.get_user(email.clone()).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials))
    };

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(auth_cookie) => auth_cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError))
    };

    let updated_jar = jar.add(auth_cookie);


    (updated_jar, Ok(StatusCode::OK.into_response()))
}


