use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
	let app = TestApp::new().await;

	let random_email = get_random_email(); // Call helper method to generate email 

	// TODO: add more malformed input test cases
	let test_cases = [
		serde_json::json!({ "password": "password123", "requires2FA": true }),
		serde_json::json!({ "email": "password123", "requires2FA": true }),
		serde_json::json!({ "requires2FA": "hello" }),
		serde_json::json!({ "email": "password123", "requires2FA": true }),
		serde_json::json!({ "email": random_email, "password": "password", "requires2FA": 1 }),
	];

	for test_case in test_cases.iter() {
		let response = app.post_signup(test_case).await;
		assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
	}
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
	let app = TestApp::new().await;

	let test_cases = [
		serde_json::json!({ "email": get_random_email(), "password": "password123", "requires2FA": true }),
		serde_json::json!({ "email": get_random_email(), "password": "password123", "requires2FA": false }),
	];

	for test_case in test_cases.iter() {
		let response = app.post_signup(test_case).await;
		assert_eq!(response.status().as_u16(), 201, "Failed for input: {:?}", test_case);
		assert_eq!(
			response.json::<auth_service::SignupResponse>().await.expect("Could not deserialize response").message,
			format!("User {} created successfully", test_case.get("email").unwrap().as_str().unwrap())
		);
	}
}
