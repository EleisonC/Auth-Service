use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use secrecy::Secret;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};

#[derive(Deserialize)]
pub struct TokenVerificationReq {
    pub token: Secret<String>
}

#[tracing::instrument(name = "Verify token", skip_all)]
pub async fn verify_token(State(state): State<AppState>, Json(request): Json<TokenVerificationReq>) -> Result<impl IntoResponse, AuthAPIError> {
    
    let valid_token = &request.token;
    let banned_tk_store = state.banned_token_store.clone();

    if validate_token(&valid_token, banned_tk_store).await.is_err() {
        return Err(AuthAPIError::InvalidToken)
    }

    Ok(StatusCode::OK)
}
