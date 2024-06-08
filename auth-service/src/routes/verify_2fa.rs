use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}};

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String
}
pub async fn verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let two_fa_code = TwoFACode::parse(request.two_fa_code.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;


    let two_fa_code_store = state.two_fa_code_store.write().await;

    let result = two_fa_code_store.get_code(&email).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if login_attempt_id != result.0 || two_fa_code != result.1 {
        return Err(AuthAPIError::IncorrectCredentials)
    }

    Ok(StatusCode::OK.into_response())
}

