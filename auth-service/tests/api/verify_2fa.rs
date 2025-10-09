use crate::helpers::TestApp;

#[tokio::test]
async fn login() {
	let app = TestApp::new().await;

	let response = app.post_verify_2fa("hello@world.com", "1", "123123").await;

	assert_eq!(response.status().as_u16(), 200);
}
