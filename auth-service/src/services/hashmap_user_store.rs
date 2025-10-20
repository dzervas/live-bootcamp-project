use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Default)]
pub struct HashmapUserStore {
	users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
	async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
		if self.users.contains_key(user.email_str()) {
			return Err(UserStoreError::UserAlreadyExists);
		}

		self.users.insert(user.email_str().to_string(), user);
		Ok(())
	}

	async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
		match self.users.get(email.as_ref()) {
			Some(user) => Ok(user.clone()),
			None => Err(UserStoreError::UserNotFound),
		}
	}

	async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
		let Ok(user) = self.get_user(email).await else {
			return Err(UserStoreError::UserNotFound);
		};

		if user.password() == password {
			Ok(())
		} else {
			Err(UserStoreError::InvalidCredentials)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_add_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::from_str("hello@example.com", "12341234", false).unwrap()).await.unwrap();
	}

	#[tokio::test]
	async fn test_get_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::from_str("hello@example.com", "12341234", false).unwrap()).await.unwrap();
		let user = store.get_user_str("hello@example.com").await.unwrap();
		assert_eq!(user.email_str(), "hello@example.com");
	}

	#[tokio::test]
	async fn test_validate_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::from_str("hello@example.com", "12341234", false).unwrap()).await.unwrap();
		let user = store.get_user_str("hello@example.com").await.unwrap();
		assert_eq!(user.email_str(), "hello@example.com");
		assert!(store.validate_user_str("hello@example.com", "12341234").await.is_ok());
		assert!(store.validate_user_str("hello@example.com", "wrong").await.is_err());
		assert!(store.validate_user_str("another@example.com", "12341234").await.is_err());
	}
}
