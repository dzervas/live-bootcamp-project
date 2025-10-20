use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
	let app = TestApp::new().await;

	// TODO: add more malformed input test cases
	let test_cases = [
		serde_json::json!({ "password": "password123"}),
		serde_json::json!({ "email": "password123"}),
		serde_json::json!({ "email": "password123" }),
	];

	for test_case in test_cases.iter() {
		let response = app.post_login(test_case).await;
		assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
	}
}

#[tokio::test]
async fn should_return_400_if_bad_user() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email 

	// TODO: add more malformed input test cases
	let test_cases = [
		serde_json::json!({ "email": "asdf", "password": "password"}),
		serde_json::json!({ "email": "", "password": "password"}),
		serde_json::json!({ "email": random_email, "password": ""}),
		serde_json::json!({ "email": random_email, "password": "1234"}),
	];

	for test_case in test_cases.iter() {
		let response = app.post_login(test_case).await;
		assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);
	}
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email 

	let user_payload = serde_json::json!({"email": random_email, "password": "password123", "requires2FA": false});
	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create normal user");

	let user_payload = serde_json::json!({"email": random_email, "password": "wrongpassword"});
	let response = app.post_login(&user_payload).await;
	assert_eq!(response.status().as_u16(), 401, "Failed for input: {:?}", user_payload);
}

#[tokio::test]
async fn should_return_204_if_valid_input() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email 
	let user_payload = serde_json::json!({"email": random_email, "password": "password123", "requires2FA": false});

	let response = app.post_signup(&user_payload).await;
	assert_eq!(response.status().as_u16(), 201, "Failed to create user");
	let login_payload = serde_json::json!({"email": random_email, "password": "password123"});
	let response = app.post_login(&login_payload).await;
	assert_eq!(response.status().as_u16(), 204, "Failed for input: {login_payload:?}");
}
