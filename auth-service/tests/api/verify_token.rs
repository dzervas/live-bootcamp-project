use reqwest::cookie::CookieStore;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
	let app = TestApp::new().await;

	let test_cases = [
		serde_json::json!({ "token": 123 }),
		serde_json::json!({}),
	];

	for test_case in test_cases.iter() {
		let response = app.post_verify_token(test_case).await;
		assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
	}
}

#[tokio::test]
async fn should_return_200_valid_token() {
	let app = TestApp::new().await;

	let user_payload = serde_json::json!({"email": "hello@world.com", "password": "password123", "requires2FA": false});
	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": "hello@world.com", "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 204, "Failed to log in");

	let header = app.cookie_jar.cookies(&"http://127.0.0.1".parse().unwrap()).unwrap();
	eprintln!("header: {header:?}");
	let token = header.to_str().unwrap().split('=').collect::<Vec<&str>>()[1];
	eprintln!("token: {token}");
	let data = serde_json::json!({"token": token});
	let response = app.post_verify_token(&data).await;
	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
	let app = TestApp::new().await;

	let test_case = serde_json::json!({"token": "invalid"});
	let response = app.post_verify_token(&test_case).await;
	assert_eq!(response.status().as_u16(), 401, "Failed for input: {:?}", test_case);
}
