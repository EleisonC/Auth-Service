use axum::http::response;

use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_a_new_user() {
    let app = TestApp::new().await;
    let params = vec![("username", "username"), ("pass", "pass")];
    let response = app.signup(params).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_user() {
    let app = TestApp::new().await;
    let params = vec![("username", "username"), ("pass", "pass")];
    let response = app.login(params).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_user() {
    let app = TestApp::new().await;
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_user() {
    let app = TestApp::new().await;
    let params = vec![("username", "username"), ("pass", "pass")];
    let response = app.verify_2fa(params).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_user() {
    let app = TestApp::new().await;
    let params = vec![("token", "token")];
    let response = app.verify_token(params).await;

    assert_eq!(response.status().as_u16(), 200);
}






