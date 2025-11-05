use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
	email: Email,
	password: Password,
	pub requires_2fa: bool,
}

impl User {
	pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
		Self {
			email,
			password,
			requires_2fa,
		}
	}

	pub fn from_str(email: &str, password: &str, requires_2fa: bool) -> Result<Self, String> {
		Ok(Self {
			email: Email::from_str(email)?,
			password: Password::from_str(password)?,
			requires_2fa,
		})
	}

	pub fn email(&self) -> Email { self.email.clone() }
	pub fn email_str(&self) -> &str { self.email.as_ref() }
	pub fn password(&self) -> Password { self.password.clone() }
	pub fn password_str(&self) -> &str { self.password.as_ref() }
	pub fn requires_2fa(&self) -> bool { self.requires_2fa }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Email(String);

impl AsRef<str> for Email {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl FromStr for Email {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.contains('@') {
			Ok(Email(s.to_string()))
		} else {
			Err("Invalid email format".to_string())
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Password(String);

impl AsRef<str> for Password {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl FromStr for Password {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() >= 8 {
			Ok(Password(s.to_string()))
		} else {
			Err("Password must be at least 8 characters long".to_string())
		}
	}
}
