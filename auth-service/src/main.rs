use std::sync::Arc;

use auth_service::Application;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
	let user_store = auth_service::HashmapUserStore::default();
	let app_state = auth_service::AppState::new(Arc::new(RwLock::new(Box::new(user_store))));

	let app = Application::build(app_state, "0.0.0.0:3000")
		.await
		.expect("Failed to build app");

	app.run().await.expect("Failed to run app");
}
