use std::sync::Arc;

use auth_service::{test, AppState, Application, BannedTokenStoreType, HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, TwoFACodeStoreType, UserStoreType};
use reqwest::cookie::Jar;
use tokio::sync::RwLock;
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
		let user_store = HashmapUserStore::default();
		let banned_token_store = HashsetBannedTokenStore::default();
		let two_fa_code_store = HashmapTwoFACodeStore::default();
		let user_store_box: UserStoreType = Arc::new(RwLock::new(Box::new(user_store)));
		let banned_token_store_box: BannedTokenStoreType = Arc::new(RwLock::new(Box::new(banned_token_store)));
		let two_fa_code_store_box: TwoFACodeStoreType = Arc::new(RwLock::new(Box::new(two_fa_code_store)));
		let app_state = AppState::new(user_store_box, banned_token_store_box.clone(), two_fa_code_store_box.clone());
		let app = Application::build(app_state, test::APP_ADDRESS)
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
			banned_token_store: banned_token_store_box,
			two_fa_code_store: two_fa_code_store_box,
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

	pub async fn post_verify_2fa(&self) -> reqwest::Response {
	// pub async fn post_verify_2fa<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
		self.http_client
			.post(format!("{}/verify-2fa", self.address))
			// .json(body)
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
