use std::str::FromStr;

use rand::{distr::Alphanumeric, Rng};

use crate::domain::{Email, Password};

use super::User;

#[async_trait::async_trait]
pub trait UserStore {
	async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
	async fn get_user(&self, email: Email) -> Result<User, UserStoreError>;
	async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError>;

	async fn get_user_str(&self, email: &str) -> Result<User, UserStoreError> {
		let email = Email::from_str(email).map_err(|_| UserStoreError::UserNotFound)?;
		self.get_user(email).await
	}

	async fn validate_user_str(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
		let email = Email::from_str(email).map_err(|_| UserStoreError::InvalidCredentials)?;
		let password = Password::from_str(password).map_err(|_| UserStoreError::InvalidCredentials)?;
		self.validate_user(email, password).await
	}
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
	UserAlreadyExists,
	UserNotFound,
	InvalidCredentials,
	UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
	async fn add(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
	async fn check(&self, token: &str) -> Result<(), BannedTokenStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
	TokenAlreadyExists,
	TokenIsBanned,
	UnexpectedError,
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
	async fn add_code(
		&mut self,
		email: Email,
		login_attempt_id: LoginAttemptId,
		code: TwoFACode,
	) -> Result<(), TwoFACodeStoreError>;
	async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
	async fn get_code(
		&self,
		email: &Email,
	) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
	LoginAttemptIdNotFound,
	UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
	pub fn parse(id: String) -> Result<Self, String> {
		// Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
		uuid::Uuid::parse_str(&id).map_err(|_| "Invalid login attempt ID".to_string())?;
		Ok(Self(id))
	}
}

impl Default for LoginAttemptId {
	fn default() -> Self {
		Self(uuid::Uuid::new_v4().to_string())
	}
}

impl AsRef<str> for LoginAttemptId {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
	pub fn parse(code: String) -> Result<Self, String> {
		// Ensure `code` is a valid 6-digit code
		if code.len() != 6 {
			return Err("Invalid code length".to_string());
		}

		if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
			return Err("Invalid code characters".to_string());
		}

		Ok(Self(code))
	}
}

impl Default for TwoFACode {
	fn default() -> Self {
		let mut rng = rand::rng();
		let code: String = (&mut rng).sample_iter(&Alphanumeric).take(6).map(char::from).collect();
		Self(code)
	}
}

impl AsRef<str> for TwoFACode {
	fn as_ref(&self) -> &str {
		&self.0
	}
}
