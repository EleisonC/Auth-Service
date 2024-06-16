use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;


#[tokio::test]
async fn should_return_400_logout_if_jwt_cookie_missing() {
    let mut app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(
        response.status().as_u16(),
        400
    );
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_logout_if_invalid_token() {
    let mut app = TestApp::new().await;

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
    );

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_logout_if_valid_cookie() {
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

    let auth_token = response.cookies().find(|cookie| cookie.name() == JWT_COOKIE_NAME).expect("No auth cookie found");

    let response = app.logout().await;

    assert_eq!(
        response.status().as_u16(),
        200
    );

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME, auth_token.value()
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;
    assert_eq!(
        response.status().as_u16(),
        401
    );
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_logout_if_called_twice_in_a_row() {
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

    let response = app.logout().await;
    assert_eq!(
        response.status().as_u16(),
        200
    );

    let response2 = app.logout().await;
    assert_eq!(
        response2.status().as_u16(),
        400
    );
    app.clean_up().await;
}

