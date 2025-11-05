use std::collections::HashMap;

use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
	codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
	async fn add_code(
		&mut self,
		email: Email,
		login_attempt_id: LoginAttemptId,
		code: TwoFACode,
	) -> Result<(), TwoFACodeStoreError> {
		if self.codes.contains_key(&email) {
			return Err(TwoFACodeStoreError::LoginAttemptIdNotFound);
		}

		self.codes.insert(email, (login_attempt_id, code));
		Ok(())
	}

	async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
		if !self.codes.contains_key(email) {
			return Err(TwoFACodeStoreError::LoginAttemptIdNotFound);
		}

		self.codes.remove(email);
		Ok(())
	}

	async fn get_code(
		&self,
		email: &Email,
	) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
		if !self.codes.contains_key(email) {
			return Err(TwoFACodeStoreError::LoginAttemptIdNotFound);
		}

		let (login_attempt_id, code) = self.codes.get(email).unwrap();
		Ok((login_attempt_id.clone(), code.clone()))
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::*;

	#[tokio::test]
	async fn should_add_code() {
		let mut store = HashmapTwoFACodeStore::default();
		let email = Email::from_str("test@example.com").unwrap();
		let login_attempt_id = LoginAttemptId::default();
		let code = TwoFACode::default();

		store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await.unwrap();

		let (login_attempt_id_2, code_2) = store.get_code(&email).await.unwrap();
		assert_eq!(login_attempt_id, login_attempt_id_2);
		assert_eq!(code, code_2);
	}
}
