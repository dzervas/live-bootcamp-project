use std::str::FromStr;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::domain::{AuthAPIError, LoginAttemptId, TwoFACode};
use crate::{AppState, Email};

pub async fn verify_2fa(State(state): State<AppState>, Json(request): Json<Verify2FARequest>) -> Result<impl IntoResponse, AuthAPIError> {
	let Ok(user_email) = Email::from_str(&request.email) else {
		return Err(AuthAPIError::InvalidCredentials);
	};
	let Ok(login_attempt_id) = LoginAttemptId::parse(request.login_attempt_id) else {
		return Err(AuthAPIError::InvalidCredentials);
	};
	let Ok(two_fa_code) = TwoFACode::parse(request.two_fa_code) else {
		return Err(AuthAPIError::InvalidCredentials);
	};

	let mut two_fa_code_store = state.two_fa_code_store.write().await;

	let code_tuple = two_fa_code_store
		.get_code(&user_email).await
		.map_err(|_| AuthAPIError::InvalidCredentials)?;

	if code_tuple.0 != login_attempt_id {
		return Err(AuthAPIError::InvalidCredentials);
	}
	if code_tuple.1 != two_fa_code {
		return Err(AuthAPIError::InvalidCredentials);
	}

	two_fa_code_store
		.remove_code(&user_email).await
		.map_err(|_| AuthAPIError::UnexpectedError)?;

	Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
	pub email: String,
	#[serde(rename = "loginAttemptId")]
	pub login_attempt_id: String,
	#[serde(rename = "2FACode")]
	pub two_fa_code: String,
}
