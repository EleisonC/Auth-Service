use crate::helpers::{self, TestApp};
use auth_service::routes::SignupResponse;

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

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = helpers::get_random_email();

    let valid_data = serde_json::json!({
        "email": random_email,
        "password": "string",
        "requires2FA": true
    });

    let response = app.signup(&valid_data).await;

    assert_eq!(response.status().as_u16(),
    201,
    "User created successfully!"
    );

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
            expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "user.mail.com",
            "password": "password123",
            "requires2FA": false
        }),
        serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": true 
        }),
        serde_json::json!({
            "email": "user_test@mail.com",
            "password": "          ",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "user_test@mail.com",
            "password": "pass",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "user_test@mail.com",
            "password": "",
            "requires2FA": true
        })
    ];

    for test_case in test_cases.iter() {
        let response = app.signup(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        )
    }
}
#[tokio::test]
async fn should_return_409_if_email() {
    let app = TestApp::new().await;
    let random_email = helpers::get_random_email();


    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    app.signup(&test_case).await;

    let response = app.signup(&test_case).await;

    assert_eq!(
        response.status().as_u16(),
        409,
        "Failed for input: {:?}",
        test_case
    )
}
