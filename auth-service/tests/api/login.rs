use axum::response;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_login_if_malformed_credentials() {
    let app = TestApp::new().await;

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
}

#[tokio::test]
async fn should_return_400_login_if_invalid_input() {
    let app = TestApp::new().await;

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

}

#[tokio::test]
async fn should_return_401_login_if_incorrect_credentials() {
    let app = TestApp::new().await;

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
}
