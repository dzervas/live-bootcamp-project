use std::sync::Arc;

use auth_service::{prod, AppState, Application, BannedTokenStoreType, UserStoreType};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
	let user_store = auth_service::HashmapUserStore::default();
	let user_store_box: UserStoreType = Arc::new(RwLock::new(Box::new(user_store)));
	let banned_token_store = auth_service::HashsetBannedTokenStore::default();
	let banned_token_store_box: BannedTokenStoreType = Arc::new(RwLock::new(Box::new(banned_token_store)));
	let app_state = AppState::new(user_store_box, banned_token_store_box);

	let app = Application::build(app_state, prod::APP_ADDRESS)
		.await
		.expect("Failed to build app");

	app.run().await.expect("Failed to run app");
}
