use std::error::Error;
use axum::{
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve, Router,
    http::{Method, StatusCode},
    Json
};
use redis::{Client, RedisResult};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::{services::ServeDir, cors::CorsLayer, trace::TraceLayer};
use app_state::AppState;
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use utils::tracing::{
    make_span_with_request_id,
    on_request,
    on_response
};

pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;
pub mod utils;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Uexpected error")
            },
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token")
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
    
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator = "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);

    let mut current = e.source();

    while let Some(cause) = current {
        let str = format!("Caused by \n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }

    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
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
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://159.223.168.210:8000".parse()?
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let app = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            // .route("/logout", post(routes::logout))
            // .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state.clone())
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response)
            );

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
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}