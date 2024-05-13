use auth_service::Application;
use axum::http::request;


pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl  TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a seprate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        let testing_app = TestApp {
            address,
            http_client
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

    pub async fn signup(&self, params :Vec<(&str, &str)>) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to signup a new user")
    }

    pub async fn login(&self, params: Vec<(&str, &str)>) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to execute request login")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request logout")
    }

    pub async fn verify_2fa(&self, params: Vec<(&str, &str)>) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to execute request verify 2fa")
    }

    pub async fn verify_token(&self, params: Vec<(&str, &str)>) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to execute request verify login")
    }
}
