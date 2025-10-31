use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use crate::domain::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::AppState;

pub async fn verify_token(
	State(state): State<AppState>,
	Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
	let claim = validate_token(&request.token).await.map_err(|_| AuthAPIError::InvalidToken)?;

	let user_store = state.user_store.read().await;
	let user_email = user_store.get_user_str(&claim.sub).await.map_err(|_| AuthAPIError::UnexpectedError)?;

	Ok(Json(user_email))
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
	pub token: String,
}
