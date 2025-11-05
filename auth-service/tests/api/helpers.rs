use std::sync::Arc;

use auth_service::{test, AppState, Application};
use reqwest::cookie::Jar;
use uuid::Uuid;

pub struct TestApp {
	pub address: String,
	pub cookie_jar: Arc<Jar>,
	pub http_client: reqwest::Client,
	pub banned_token_store: auth_service::BannedTokenStoreType,
	pub two_fa_code_store: auth_service::TwoFACodeStoreType,
}

impl TestApp {
	pub async fn new() -> Self {
		let state = AppState::default();
		let banned_token_store = state.banned_token_store.clone();
		let two_fa_code_store = state.two_fa_code_store.clone();
		let app = Application::build(state, test::APP_ADDRESS)
			.await
			.expect("Failed to build app");

		let address = format!("http://{}", app.address.clone());

		// Run the auth service in a separate async task
		// to avoid blocking the main test thread.
		#[allow(clippy::let_underscore_future)]
		let _ = tokio::spawn(app.run());

		let cookie_jar = Arc::new(Jar::default());
		let http_client = reqwest::Client::builder()
			.cookie_provider(Arc::clone(&cookie_jar))
			.build()
			.unwrap();

		Self {
			address,
			cookie_jar,
			http_client,
			banned_token_store,
			two_fa_code_store,
		}
	}

	pub async fn get_root(&self) -> reqwest::Response {
		self.http_client
			.get(format!("{}/", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	pub async fn post_signup<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
		self.http_client
			.post(format!("{}/signup", self.address))
			.json(body)
			.send()
			.await
			.expect("Failed to execute request.")
	}

	pub async fn post_login<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
		self.http_client
			.post(format!("{}/login", self.address))
			.json(body)
			.send()
			.await
			.expect("Failed to execute request.")
	}

	pub async fn post_verify_2fa<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
		self.http_client
			.post(format!("{}/verify-2fa", self.address))
			.json(body)
			.send()
			.await
			.expect("Failed to execute request.")
	}

	pub async fn post_logout(&self) -> reqwest::Response {
		self.http_client
			.post(format!("{}/logout", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	pub async fn post_verify_token<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
		self.http_client
			.post(format!("{}/verify-token", self.address))
			.json(body)
			.send()
			.await
			.expect("Failed to execute request.")
	}
}

pub fn get_random_email() -> String {
	format!("{}@example.com", Uuid::new_v4())
}
