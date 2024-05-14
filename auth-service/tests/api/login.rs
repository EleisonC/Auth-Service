use crate::helpers::TestApp;

#[tokio::test]
async fn login_user() {
    let app = TestApp::new().await;
    let params = vec![("username", "username"), ("pass", "pass")];
    let response = app.login(&params).await;

    assert_eq!(response.status().as_u16(), 200);
}