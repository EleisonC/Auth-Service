use std::error::Error;
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    serve::Serve, Router,
    http::StatusCode
};
use tower_http::services::ServeDir;
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router difinition from main.rs to here 
        // Also, remove the `hello` route
        // we dont need it at this point!
        let app = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", get(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token));

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

async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
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
