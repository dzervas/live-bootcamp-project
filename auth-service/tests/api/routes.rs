use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
	let app = TestApp::new().await;

	let response = app.get_root().await;

	assert_eq!(response.status().as_u16(), 200);
	assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup() {
	let app = TestApp::new().await;

	let response = app.post_signup("hello@world.com", "hello123", false).await;

	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login() {
	let app = TestApp::new().await;

	let response = app.post_login("hello@world.com", "hello123").await;

	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa() {
	let app = TestApp::new().await;

	let response = app.post_verify_2fa("hello@world.com", "1", "123123").await;

	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout() {
	let app = TestApp::new().await;

	let response = app.post_logout("myjwt").await;

	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token() {
	let app = TestApp::new().await;

	let response = app.post_verify_token("myjwt").await;

	assert_eq!(response.status().as_u16(), 200);
}
