use reqwest::{cookie::CookieStore, Url};

use crate::helpers::TestApp;
use auth_service::JWT_COOKIE_NAME;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
	let app = TestApp::new().await;

	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
	let app = TestApp::new().await;

	app.cookie_jar.add_cookie_str(
		&format!("{JWT_COOKIE_NAME}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/"),
		&Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
	);

	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
	let app = TestApp::new().await;

	let user_payload = serde_json::json!({"email": "hello@world.com", "password": "password123", "requires2FA": false});
	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": "hello@world.com", "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 204, "Failed to log in");

	let header = app.cookie_jar.cookies(&"http://127.0.0.1".parse().unwrap()).unwrap();
	let token = header.to_str().unwrap().split('=').collect::<Vec<&str>>()[1];

	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 200);

	let banned_store = app.banned_token_store.read().await;
	assert!(banned_store.check(token).await.is_err());
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
	let app = TestApp::new().await;

	let user_payload = serde_json::json!({"email": "hello@world.com", "password": "password123", "requires2FA": false});
	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": "hello@world.com", "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 204, "Failed to log in");

	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 200);

	let response = app.post_logout().await;
	assert_eq!(response.status().as_u16(), 400);
}
