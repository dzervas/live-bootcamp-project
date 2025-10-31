use axum::extract::State;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;

use crate::domain::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::{AppState, JWT_COOKIE_NAME};

pub async fn logout(
	State(state): State<AppState>,
	jar: CookieJar
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
	let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

	let token = cookie.value();
	validate_token(token, &state.banned_token_store).await.map_err(|_| AuthAPIError::InvalidToken)?;
	state.banned_token_store.write()
		.await
		.add(token.to_string())
		.await
		.map_err(|_| AuthAPIError::InvalidToken)?;

	Ok((jar.remove(JWT_COOKIE_NAME), StatusCode::OK))
}
