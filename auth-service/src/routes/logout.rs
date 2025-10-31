use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;

use crate::domain::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::JWT_COOKIE_NAME;

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
	let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

	let token = cookie.value();
	validate_token(token).await.map_err(|_| AuthAPIError::InvalidToken)?;

	Ok((jar.remove(JWT_COOKIE_NAME), StatusCode::OK))
}
