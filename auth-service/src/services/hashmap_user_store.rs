use std::collections::HashMap;

use crate::domain::{User, UserStore, UserStoreError};

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
		if self.users.contains_key(user.email()) {
			return Err(UserStoreError::UserAlreadyExists);
		}

		self.users.insert(user.email().to_string(), user);
		Ok(())
	}

	async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
		match self.users.get(email) {
			Some(user) => Ok(user.clone()),
			None => Err(UserStoreError::UserNotFound),
		}
	}

	async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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
		store.add_user(User::new("hello@example.com".to_owned(), "1234".to_owned(), false)).await.unwrap();
	}

	#[tokio::test]
	async fn test_get_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::new("hello@example.com".to_owned(), "1234".to_owned(), false)).await.unwrap();
		let user = store.get_user("hello@example.com").await.unwrap();
		assert_eq!(user.email(), "hello@example.com");
	}

	#[tokio::test]
	async fn test_validate_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::new("hello@example.com".to_owned(), "1234".to_owned(), false)).await.unwrap();
		let user = store.get_user("hello@example.com").await.unwrap();
		assert_eq!(user.email(), "hello@example.com");
		assert!(store.validate_user("hello@example.com", "1234").await.is_ok());
		assert!(store.validate_user("hello@example.com", "wrong").await.is_err());
		assert!(store.validate_user("another@example.com", "1234").await.is_err());
	}
}
