use axum::{http::StatusCode, extract::State,
    response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, User, UserStoreError, Password}};
#[derive(Deserialize)]
pub struct SignupRequest {
    pub email:String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String
}

#[tracing::instrument(name = "Signup", skip_all, err(Debug))]
pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email.clone(), password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    match user_store.add_user(user).await {
        Ok(()) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string()
            });
            
            Ok((StatusCode::CREATED, response))
        },
        Err(_) => Err(AuthAPIError::UnexpectedError),
    }
}
