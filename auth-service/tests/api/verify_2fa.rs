use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_user() {
    let app = TestApp::new().await;
    let params = vec![("username", "username"), ("pass", "pass")];
    let response = app.verify_2fa(&params).await;

    assert_eq!(response.status().as_u16(), 200);
}
