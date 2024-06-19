use axum::{extract::State, http::{response, StatusCode}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
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

    let user_store = state.user_store.read().await;

    if user_store.validate_user(email.clone(), password.clone()).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials))
    }

    let user = match user_store.get_user(email.clone()).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials))
    };

    match user.requires2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await
    }
}


async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let mut store = state.two_fa_code_store.write().await;
    if store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError))
    }

    let email_client = state.email_client.write().await;

    if email_client.send_email(email, login_attempt_id.as_ref(), two_fa_code.as_ref()).await.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError))
    }

    let two_factor = TwoFactorAuthResponse {
        message: "2FA required".to_string(),
        login_attempt_id: login_attempt_id.as_ref().to_string()
    };

    let response = Json(LoginResponse::TwoFactorAuth(two_factor));
    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>
) {
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(auth_cookie) => auth_cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError))
    };

    let updated_jar = jar.add(auth_cookie);

    let response = Json(LoginResponse::RegularAuth);
    (updated_jar, Ok((StatusCode::OK, response)))
}


