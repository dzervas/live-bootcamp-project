use std::str::FromStr;

use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::State;
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::domain::{AuthAPIError, Email, Password};
use crate::AppState;

pub async fn login(
	State(state): State<AppState>,
	jar: CookieJar,
	Json(request): Json<LoginRequest>
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
	let user_store = state.user_store.read().await;
	let Ok(user_email) = Email::from_str(&request.email) else {
		return Err(AuthAPIError::InvalidCredentials);
	};
	let Ok(user_password) = Password::from_str(&request.password) else {
		return Err(AuthAPIError::InvalidCredentials);
	};

	if user_store.validate_user(user_email.clone(), user_password).await.is_err() {
		return Err(AuthAPIError::IncorrectPassword);
	}

	let auth_cookie = crate::utils::auth::generate_auth_cookie(&user_email)
		.map_err(|_| AuthAPIError::TokenCreationError)?;

	let updated_jar = jar.add(auth_cookie);

	Ok((updated_jar, StatusCode::NO_CONTENT.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}
