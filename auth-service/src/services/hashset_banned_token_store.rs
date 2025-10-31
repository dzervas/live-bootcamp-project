use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};


#[derive(Default)]
pub struct HashsetBannedTokenStore {
	tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
	async fn add(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
		self.check(&token).await?;
		self.tokens.insert(token);
		Ok(())
	}

	async fn check(&self, token: &str) -> Result<(), BannedTokenStoreError> {
		if self.tokens.contains(token) {
			Err(BannedTokenStoreError::TokenIsBanned)
		} else {
			Ok(())
		}
	}
}
