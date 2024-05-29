use auth_service::utils::constants::JWT_COOKIE_NAME;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_verifytk_if_malformed_input() {
    let app = TestApp::new().await;

    let valid_token = serde_json::json!({
        "verified_token": "auth_token.value()"
    });

    let response = app.verify_token(&valid_token).await;

    assert_eq!(
        response.status().as_u16(),
        422
    );

}

#[tokio::test]
async fn should_return_200_verifytk_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_user = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.signup(&valid_user).await;

    assert_eq!(
        response.status().as_u16(),
        201
    );

    let valid_user_lg = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.login(&valid_user_lg).await;
    assert_eq!(
        response.status().as_u16(),
        200
    );

    let auth_token = response.cookies().find(|cookie| cookie.name() == JWT_COOKIE_NAME).expect("No auth cookie found");

    let valid_token = serde_json::json!({
        "token": auth_token.value()
    });

    let response = app.verify_token(&valid_token).await;

    assert_eq!(
        response.status().as_u16(),
        200
    );
}

#[tokio::test]
async fn should_return_401_verifytk_invalid_token() {
    let app = TestApp::new().await;

    let valid_token = serde_json::json!({
        "token": "auth_token.value()"
    });

    let response = app.verify_token(&valid_token).await;

    assert_eq!(
        response.status().as_u16(),
        401
    );
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let valid_user = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.signup(&valid_user).await;

    assert_eq!(
        response.status().as_u16(),
        201
    );

    let valid_user_lg = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.login(&valid_user_lg).await;
    assert_eq!(
        response.status().as_u16(),
        200
    );

    let auth_token = response.cookies().find(|cookie| cookie.name() == JWT_COOKIE_NAME).expect("No auth cookie found");

    let valid_token = serde_json::json!({
        "token": auth_token.value()
    });

    let response = app.verify_token(&valid_token).await;

    assert_eq!(
        response.status().as_u16(),
        200
    );

    let response = app.logout().await;

    assert_eq!(
        response.status().as_u16(),
        200
    );

    let response = app.verify_token(&valid_token).await;

    assert_eq!(
        response.status().as_u16(),
        401
    );
}
