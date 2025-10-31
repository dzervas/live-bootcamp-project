use std::str::FromStr;

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
