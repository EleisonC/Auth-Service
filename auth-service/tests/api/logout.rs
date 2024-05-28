use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use axum::http::response;
use reqwest::Url;


#[tokio::test]
async fn should_return_400_logout_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    // let random_email = get_random_email();

    // let valid_signup_body = serde_json::json!({
    //     "email": random_email,
    //     "password": "password123",
    //     "requires2FA": false
    // });

    // let response = app.signup(&valid_signup_body).await;

    // assert_eq!(
    //     response.status().as_u16(),
    //     201
    // );

    // let login_body = serde_json::json!({
    //     "email": random_email,
    //     "password": "password123"
    // });

    // let response = app.login(&login_body).await;

    // assert_eq!(
    //     response.status().as_u16(),
    //     200
    // );

    let response = app.logout().await;

    assert_eq!(
        response.status().as_u16(),
        400
    )
}

#[tokio::test]
async fn should_return_401_logout_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;
    assert_eq!(
        response.status().as_u16(),
        401
    )
}

#[tokio::test]
async fn should_return_200_logout_if_valid_cookie() {
    let app = TestApp::new().await;

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

    let response = app.logout().await;

    assert_eq!(
        response.status().as_u16(),
        200
    )
}

#[tokio::test]
async fn should_return_400_logout_if_called_twice_in_a_row() {
    let app = TestApp::new().await;

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

    let response = app.logout().await;
    assert_eq!(
        response.status().as_u16(),
        200
    );

    let response2 = app.logout().await;
    assert_eq!(
        response2.status().as_u16(),
        400
    )
}

