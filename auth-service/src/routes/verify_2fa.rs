use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}, utils::auth::generate_auth_cookie};

use super::LoginResponse;

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String
}
#[tracing::instrument(name = "Verify 2FA", skip_all)]
pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials))
    };
    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id.clone()) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials))
    };
    let two_fa_code = match TwoFACode::parse(request.two_fa_code.clone()) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials))
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    let result = match two_fa_code_store.get_code(&email).await {
        Ok(result) => result,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials))
    };

    if login_attempt_id != result.0 || two_fa_code != result.1 {
        return (jar, Err(AuthAPIError::IncorrectCredentials))
    }

    if  let Err(e) = two_fa_code_store.remove_code(&email).await {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())))
    }

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(auth_cookie) => auth_cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(e)))
    };

    let updated_jar = jar.add(auth_cookie);

    let response = Json(LoginResponse::RegularAuth);

    (updated_jar, Ok((StatusCode::OK, response)))
}

