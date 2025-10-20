use std::str::FromStr;

use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, Password};
use crate::AppState;

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
	let user_store = state.user_store.read().await;
	let Ok(user_email) = Email::from_str(&request.email) else {
		return Err(AuthAPIError::InvalidCredentials);
	};
	let Ok(user_password) = Password::from_str(&request.password) else {
		return Err(AuthAPIError::InvalidCredentials);
	};

	if user_store.validate_user(user_email, user_password).await.is_err() {
		return Err(AuthAPIError::IncorrectPassword);
	}

	// let user = user_store.get_user(&request.email).await;

	let response = LoginResponse {
		message: format!("User {} logged in successfully", request.email),
	};

	Ok((StatusCode::NO_CONTENT, Json(response)))
}

#[derive(Deserialize)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
	pub message: String,
}
