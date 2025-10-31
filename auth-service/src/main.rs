use std::sync::Arc;

use auth_service::{prod, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
	let user_store = auth_service::HashmapUserStore::default();
	let app_state = auth_service::AppState::new(Arc::new(RwLock::new(Box::new(user_store))));

	let app = Application::build(app_state, prod::APP_ADDRESS)
		.await
		.expect("Failed to build app");

	app.run().await.expect("Failed to run app");
}
