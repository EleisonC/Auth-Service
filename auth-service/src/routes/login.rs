use axum::{http::StatusCode, extract::State,
    response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password}};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}


pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(email.clone(), password.clone()).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials)
    }

    let _user = user_store.get_user(email).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;

    Ok(StatusCode::OK.into_response())
}


