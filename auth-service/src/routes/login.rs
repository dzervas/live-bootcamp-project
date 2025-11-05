use std::str::FromStr;

use axum::Json;
use axum::response::{IntoResponse, Response};
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode, TwoFACodeStore};
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

	let user = user_store.get_user(user_email.clone()).await.map_err(|_| AuthAPIError::InvalidCredentials)?;

	if user.requires_2fa {
		handle_2fa(user_email, &state, jar).await
	} else {
		handle_no_2fa(&user_email, jar).await
	}
}

async fn handle_2fa(email: Email, state: &AppState, jar: CookieJar) -> Result<(CookieJar, Response), AuthAPIError> {
	let login_attempt_id = LoginAttemptId::default();
	let two_fa_code = TwoFACode::default();

	state.two_fa_code_store
		.write().await
		.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await
		.map_err(|_| AuthAPIError::UnexpectedError)?;

	let twofa_response = LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
		message: "2FA required".to_string(),
		login_attempt_id: two_fa_code.as_ref().to_string(),
	});

	Ok((jar, (StatusCode::PARTIAL_CONTENT, Json(twofa_response)).into_response()))
}

async fn handle_no_2fa(email: &Email, jar: CookieJar) -> Result<(CookieJar, Response), AuthAPIError> {
	let auth_cookie = crate::utils::auth::generate_auth_cookie(email)
		.map_err(|_| AuthAPIError::TokenCreationError)?;

	let updated_jar = jar.add(auth_cookie);

	Ok((updated_jar, StatusCode::NO_CONTENT.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
