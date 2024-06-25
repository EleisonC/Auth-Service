use auth_service::{domain::{Email, TwoFACodeStore}, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME
};
use secrecy::{ExposeSecret, Secret};

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_login_if_malformed_credentials() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_data = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response1 = app.signup(&valid_data).await;
    assert_eq!(response1.status().as_u16(),
        201,
        "User created successfully!"
        );
        let invalid_login_data = [    
            serde_json::json!({
            "email": random_email,
            }),
            serde_json::json!({
                "emal": random_email,
                "pass": "password123"
            }),
            serde_json::json!({
                "password": random_email,
            }),
        ];
    // let response2 = app.login(&valid_login_data)
    for invalid_data in invalid_login_data.iter() {
        let response2 = app.login(&invalid_data).await;
        assert_eq!(
            response2.status().as_u16(),
            422,
            "Failed for input: {:?}",
            invalid_data
        )
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_login_if_invalid_input() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_data = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response1 = app.signup(&valid_data).await;
    assert_eq!(response1.status().as_u16(),
        201,
        "User created successfully!"
        );
    
    let invalid_login_data = [    
        serde_json::json!({
        "email": "     @     .com",
        "password": "password123"
        }),
        serde_json::json!({
            "email": "",
            "password": ""
        }),
        serde_json::json!({
            "password": "123",
            "email": "dsada@mail.com"
        }),
    ];

    for invalid_data in invalid_login_data {
        let response2 = app.login(&invalid_data).await;

        assert_eq!(
            response2.status().as_u16(), 
            400, 
            "Failed for input {:?}", 
            invalid_data
        )
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_login_if_incorrect_credentials() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_data = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response1 = app.signup(&valid_data).await;
    assert_eq!(
        response1.status().as_u16(),
        201,
        "User created successfully!"
    );

    let incorrect_creds = [
        serde_json::json!({
            "email": random_email,
            "password": "password321",
        }),
        serde_json::json!({
            "email": "wert@mail.com",
            "password": "password123"
        })
    ];

    for data in incorrect_creds.iter() {
        let response2 = app.login(&data).await;

        assert_eq!(
            response2.status().as_u16(),
            401,
            "Failed for input: {:?}",
            data
        )
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_login_if_valid_credentials_and_2fa_disabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.signup(&valid_signup_body).await;

    assert_eq!(
        response.status().as_u16(),
        201
    );

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.login(&login_body).await;

    assert_eq!(
        response.status().as_u16(),
        200
    );

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.signup(&valid_signup_body).await;

    assert_eq!(
        response.status().as_u16(),
        201
    );

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.login(&login_body).await;

    assert_eq!(
        response.status().as_u16(),
        206
    );

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await.expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let binding = app.two_fa_code_store.clone();
    let test_store = binding.read().await;
    let email = Email::parse(Secret::new(random_email)).unwrap();
    let result = test_store.get_code(&email).await.unwrap();

    assert_eq!(result.0.as_ref().expose_secret().to_owned(), json_body.login_attempt_id);
    app.clean_up().await;
}