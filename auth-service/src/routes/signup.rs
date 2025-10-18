use std::str::FromStr as _;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, Password, User};
use crate::AppState;

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {
	let Ok(user) = request.to_user() else {
		return Err(AuthAPIError::InvalidCredentials);
	};
	let mut user_store = state.user_store.write().await;

	let response = SignupResponse {
		message: format!("User {} created successfully", user.email()),
	};

	if user_store.get_user(user.email()).await.is_ok() {
		return Err(AuthAPIError::UserAlreadyExists);
	}

	if user_store.add_user(user).await.is_err() {
		return Err(AuthAPIError::UnexpectedError);
	}

	Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Deserialize)]
pub struct SignupRequest {
	pub email: String,
	pub password: String,
	#[serde(rename = "requires2FA")]
	pub requires_2fa: bool,
}

impl SignupRequest{
	pub fn to_user(&self) -> Result<User, String> {
		Ok(User::new(Email::from_str(&self.email)?, Password::from_str(&self.password)?, self.requires_2fa))
	}
}

#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
	pub message: String,
}
