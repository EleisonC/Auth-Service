use auth_service::{app_state::AppState, get_postgres_pool, get_redis_client, services::{self, HashmapTwoFACodeStore}, utils::constants::{test, DATABASE_URL, DEFAULT_REDIS_HOSTNAME}, Application};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use reqwest::cookie::Jar;


pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub db_name: String,
    pub cookie_jar: Arc<Jar>,
    pub two_fa_code_store: Arc<RwLock<HashmapTwoFACodeStore>>,
    clean_up_called: bool
}

impl  TestApp {
    pub async fn new() -> Self {
        let db_name = Uuid::new_v4().to_string();
        let pg_pool = configure_postgresql(&db_name).await;
        let redis_client = Arc::new(RwLock::new(configure_redis()));

        let test_user_store = Arc::new(RwLock::new(services::PostgresUserStore::new(pg_pool)));
        let test_banned_token_store = Arc::new(RwLock::new(services::RedisBannedTokenStore::new(redis_client)));
        let two_fa_code_store = Arc::new(RwLock::new(services::HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(services::MockEmailClient::default()));

        let test_app_state = AppState::new(test_user_store, test_banned_token_store, two_fa_code_store.clone(), email_client);
        let app = Application::build(test_app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a seprate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        let testing_app = TestApp {
            address,
            http_client,
            db_name,
            cookie_jar,
            two_fa_code_store,
            clean_up_called: false
        };

        testing_app
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn signup<Body>(&self, body: &Body) -> reqwest::Response
    where
    Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to signup a new user")
    }

    pub async fn login<Body>(&self, body: &Body) -> reqwest::Response
    where
    Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request login")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request logout")
    }

    pub async fn verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
    Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request verify 2fa")
    }

    pub async fn verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
    Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request verify login")
    }

    pub async fn clean_up(&mut self) {
        delete_database(&self.db_name).await;
        self.clean_up_called = true;
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("You must clean up db: {}", self.db_name)
        } else {
            println!("Dropping Test DB: {}", self.db_name);
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql(db_name: &str) -> PgPool {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    configure_database(&postgresql_conn_url, &db_name).await;

    let postgres_conn_url_with_db = format!("{}/{}",
        postgresql_conn_url, db_name);

    get_postgres_pool(&postgres_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");


    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database")
}


async fn delete_database(db_name: &str) {
    let postgresql_conn_url: String = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                AND pid <> pg_backend_pid();
                "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

        connection
            .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
            .await
            .expect("Failed to drop the database.");
}

fn configure_redis() -> redis::Connection {
    let redis_hostname = DEFAULT_REDIS_HOSTNAME.to_owned();

    get_redis_client(redis_hostname)
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
