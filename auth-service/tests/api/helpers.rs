use auth_service::Application;

pub struct TestApp {
	pub address: String,
	pub http_client: reqwest::Client,
}

impl TestApp {
	pub async fn new() -> Self {
		let app = Application::build("127.0.0.1:0")
			.await
			.expect("Failed to build app");

		let address = format!("http://{}", app.address.clone());

		// Run the auth service in a separate async task
		// to avoid blocking the main test thread.
		#[allow(clippy::let_underscore_future)]
		let _ = tokio::spawn(app.run());

		let http_client = reqwest::Client::new();

		Self {
			http_client,
			address
		}
	}

	pub async fn get_root(&self) -> reqwest::Response {
		self.http_client
			.get(format!("{}/", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	#[allow(unused_variables)]
	pub async fn post_signup(&self, email: &'static str, password: &'static str, requires_2fa: bool) -> reqwest::Response {
		self.http_client
			.post(format!("{}/signup", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	#[allow(unused_variables)]
	pub async fn post_login(&self, email: &'static str, password: &'static str) -> reqwest::Response {
		self.http_client
			.post(format!("{}/login", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	#[allow(unused_variables)]
	pub async fn post_verify_2fa(&self, email: &'static str, login_attempt_id: &'static str, twofa_code: &'static str) -> reqwest::Response {
		self.http_client
			.post(format!("{}/verify-2fa", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	#[allow(unused_variables)]
	pub async fn post_logout(&self, jwt: &'static str) -> reqwest::Response {
		self.http_client
			.post(format!("{}/logout", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}

	#[allow(unused_variables)]
	pub async fn post_verify_token(&self, jwt: &'static str) -> reqwest::Response {
		self.http_client
			.post(format!("{}/verify-token", self.address))
			.send()
			.await
			.expect("Failed to execute request.")
	}
}
