use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::User;
use crate::AppState;

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> impl IntoResponse {
	let user = request.to_user().unwrap();
	let mut user_store = state.user_store.write().await;

	let response = SignupResponse {
		message: format!("User {} created successfully", user.email()),
	};

	user_store.add_user(user).unwrap();

	(StatusCode::CREATED, Json(response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
	pub email: String,
	pub password: String,
	#[serde(rename = "requires2FA")]
	pub requires_2fa: bool,
}

impl SignupRequest{
	pub fn validate(&self) -> Result<(), String> {
		if self.email.is_empty() || self.password.is_empty() {
			return Err("Email and password cannot be empty".to_string());
		}
		if !self.email.contains('@') {
			return Err("Invalid email format".to_string());
		}
		if self.password.len() < 8 {
			return Err("Password must be at least 8 characters long".to_string());
		}
		Ok(())
	}

	pub fn to_user(&self) -> Result<User, String> {
		self.validate()?;
		Ok(User::new(self.email.clone(), self.password.clone(), self.requires_2fa))
	}
}

#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
	pub message: String,
}
