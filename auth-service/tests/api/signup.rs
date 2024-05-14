use crate::helpers::{self, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = helpers::get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password321",
            "requires2FA": true
        }),
        serde_json::json!({
            "password": "password321",
            "email": random_email
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "user@example.com",
            "password": "string",
            "requires2fa": true
        })
    ];

    for test_case in test_cases.iter() {
        let response = app.signup(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        )
    }
}

