use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
	UserAlreadyExists,
	UserNotFound,
	InvalidCredentials,
	UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Default)]
pub struct HashmapUserStore {
	users: HashMap<String, User>,
}

impl HashmapUserStore {
	pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
		// Return `UserStoreError::UserAlreadyExists` if the user already exists,
		// otherwise insert the user into the hashmap and return `Ok(())`.
		if self.users.contains_key(user.email()) {
			return Err(UserStoreError::UserAlreadyExists);
		}

		self.users.insert(user.email().to_string(), user);
		Ok(())
	}

	pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
		match self.users.get(email) {
			Some(user) => Ok(user.clone()),
			None => Err(UserStoreError::UserNotFound),
		}
	}

	pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
		let Ok(user) = self.get_user(email) else {
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
		store.add_user(User::new("hello@example.com".to_owned(), "1234".to_owned(), false)).unwrap();
	}

	#[tokio::test]
	async fn test_get_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::new("hello@example.com".to_owned(), "1234".to_owned(), false)).unwrap();
		let user = store.get_user("hello@example.com").unwrap();
		assert_eq!(user.email(), "hello@example.com");
	}

	#[tokio::test]
	async fn test_validate_user() {
		let mut store = HashmapUserStore::default();
		store.add_user(User::new("hello@example.com".to_owned(), "1234".to_owned(), false)).unwrap();
		let user = store.get_user("hello@example.com").unwrap();
		assert_eq!(user.email(), "hello@example.com");
		assert!(store.validate_user("hello@example.com", "1234").is_ok());
		assert!(store.validate_user("hello@example.com", "wrong").is_err());
		assert!(store.validate_user("another@example.com", "1234").is_err());
	}
}
