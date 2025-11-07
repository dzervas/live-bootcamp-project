use std::str::FromStr;

use auth_service::{Email, TwoFactorAuthResponse};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_correct_code() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email
	let user_payload = serde_json::json!({"email": random_email, "password": "password123", "requires2FA": true});

	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": random_email, "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 206);
	// let login = response.json::<TwoFactorAuthResponse>().await.unwrap();
	let (login_attempt_id, code) = {
		let two_fa_code_store = app.two_fa_code_store.read().await;
		two_fa_code_store.get_code(&Email::from_str(&random_email).unwrap()).await.unwrap()
	};
	let correct_2fa_code = serde_json::json!({"email": random_email, "loginAttemptId": login_attempt_id.as_ref(), "2FACode": code.as_ref()});
	let response = app.post_verify_2fa(&correct_2fa_code).await;
	assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
	let app = TestApp::new().await;

	let test_cases = [
		serde_json::json!({ "email": "hello@world.com", "loginAttemptId": 1234, "2FACode": "123456"}),
		serde_json::json!({ "email": "hello@world.com", "loginAttemptId": "1234", "2FACode": 123456}),
		serde_json::json!({ "email": "hello@world.com", "2FACode": "123456"}),
		serde_json::json!({ "email": "hello@world.com", "loginAttemptId": "123456"}),
		serde_json::json!({ "loginAttemptId": "1234", "2FACode": "123456"}),
	];

	for test_case in test_cases.iter() {
		let response = app.post_verify_2fa(test_case).await;
		assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
	}
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email

	// TODO: add more malformed input test cases
	let test_cases = [
		serde_json::json!({ "email": "asdf", "loginAttemptId": "1234", "2FACode": "123456"}),
		serde_json::json!({ "email": "", "loginAttemptId": "1234", "2FACode": "123456"}),
		serde_json::json!({ "email": random_email, "loginAttemptId": "1234", "2FACode": "123"}),
		serde_json::json!({ "email": random_email, "loginAttemptId": "hello", "2FACode": "123456"}),
	];

	for test_case in test_cases.iter() {
		let response = app.post_verify_2fa(test_case).await;
		assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);
	}
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email
	let user_payload = serde_json::json!({"email": random_email, "password": "password123", "requires2FA": true});

	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": random_email, "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 206);
	let login = response.json::<TwoFactorAuthResponse>().await.unwrap();
	let incorrect_2fa_code = serde_json::json!({"email": random_email, "loginAttemptId": login.login_attempt_id, "2FACode": "123456"});
	let response = app.post_verify_2fa(&incorrect_2fa_code).await;
	assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email
	let user_payload = serde_json::json!({"email": random_email, "password": "password123", "requires2FA": true});

	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": random_email, "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 206);
	// let login = response.json::<TwoFactorAuthResponse>().await.unwrap();
	let (login_attempt_id, code) = {
		let two_fa_code_store = app.two_fa_code_store.read().await;
		two_fa_code_store.get_code(&Email::from_str(&random_email).unwrap()).await.unwrap()
	};
	let correct_2fa_code = serde_json::json!({"email": random_email, "loginAttemptId": login_attempt_id.as_ref(), "2FACode": code.as_ref()});
	let response = app.post_verify_2fa(&correct_2fa_code).await;
	assert_eq!(response.status().as_u16(), 200);
	let response = app.post_verify_2fa(&correct_2fa_code).await;
	assert_eq!(response.status().as_u16(), 401);
}
