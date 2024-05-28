use auth_service::{app_state::AppState, services, Application, utils::constants::test};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::cookie::Jar;


pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>
}

impl  TestApp {
    pub async fn new() -> Self {
        let test_user_store = services::HashmapUserStore::default();
        let test_app_state = AppState::new(Arc::new(RwLock::new(test_user_store)));
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
            cookie_jar
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
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
