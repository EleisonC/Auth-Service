use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::AuthAPIError, utils::auth::validate_token};

#[derive(Deserialize)]
pub struct TokenVerificationReq {
    pub token: String
}

pub async fn verify_token(Json(request): Json<TokenVerificationReq>) -> Result<impl IntoResponse, AuthAPIError> {
    
    let valid_token = &request.token;

    if validate_token(&valid_token).await.is_err() {
        return Err(AuthAPIError::InvalidToken)
    }

    Ok(StatusCode::OK.into_response())
}
