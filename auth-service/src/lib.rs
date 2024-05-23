use std::error::Error;
use axum::{
    response::{IntoResponse, Response},
    routing::{get, post},
    serve::Serve, Router,
    http::StatusCode,
    Json
};
use tower_http::services::ServeDir;
use app_state::AppState;
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};

pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Uexpected error")
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
    
        (status, body).into_response()
    }
}

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router difinition from main.rs to here 
        // Also, remove the `hello` route
        // we dont need it at this point!
        let app = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(login))
            .route("/logout", get(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state.clone());

        let router = app;

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        let app_inst = Application {
            server,
            address
        };

        Ok(app_inst)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
