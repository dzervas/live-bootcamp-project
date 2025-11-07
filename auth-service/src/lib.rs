use std::error::Error;

use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::serve::Serve;
use axum::http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod app_state;
mod domain;
mod routes;
mod services;
mod utils;

pub use app_state::*;
pub use routes::signup::SignupResponse;
pub use services::hashmap_user_store::HashmapUserStore;
pub use services::hashset_banned_token_store::HashsetBannedTokenStore;
pub use services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
pub use services::mock_email_client::MockEmailClient;
pub use utils::constants::*;
pub use routes::login::TwoFactorAuthResponse;
pub use domain::Email;

use crate::domain::AuthAPIError;

// This struct encapsulates our application-related logic.
pub struct Application {
	server: Serve<Router, Router>,
	// address is exposed as a public field
	// so we have access to it in tests.
	pub address: String,
}

impl Application {
	pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
		let allowed_origins = [
			"http://localhost:3000".parse()?,
			"http://127.0.0.1:3000".parse()?,
		];

		let cors = CorsLayer::new()
			.allow_methods([Method::GET, Method::POST])
			.allow_origin(allowed_origins)
			.allow_credentials(true);

		let router = Router::new()
			.nest_service("/", ServeDir::new("assets"))
			.route("/signup", post(routes::signup))
			.route("/login", post(routes::login))
			.route("/verify-2fa", post(routes::verify_2fa))
			.route("/logout", post(routes::logout))
			.route("/verify-token", post(routes::verify_token))
			.with_state(app_state)
			.layer(cors);

		let listener = tokio::net::TcpListener::bind(address).await?;
		let address = listener.local_addr()?.to_string();
		let server = axum::serve(listener, router);

		Ok(Self {
			server,
			address,
		})
	}

	pub async fn run(self) -> Result<(), std::io::Error> {
		println!("listening on {}", &self.address);
		self.server.await
	}
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
	pub error: String,
}

impl IntoResponse for AuthAPIError {
	fn into_response(self) -> Response {
		let (status, error_message) = match self {
			AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
			AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
			AuthAPIError::IncorrectPassword => (StatusCode::UNAUTHORIZED, "The password is incorrect or the user does not exist"),
			AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
			AuthAPIError::TokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create authentication token"),
			AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing authentication token"),
			AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
			AuthAPIError::Invalid2FACredentials => (StatusCode::UNAUTHORIZED, "Invalid 2FA code or login attempt ID"),
		};
		let body = Json(ErrorResponse {
			error: error_message.to_string(),
		});
		(status, body).into_response()
	}
}
