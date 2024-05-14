use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_user() {
    let app = TestApp::new().await;
    let params = vec![("token", "token")];
    let response = app.verify_token(&params).await;

    assert_eq!(response.status().as_u16(), 200);
}