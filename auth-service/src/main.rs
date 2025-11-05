use auth_service::{prod, AppState, Application};

#[tokio::main]
async fn main() {
	let app_state = AppState::default();

	let app = Application::build(app_state, prod::APP_ADDRESS)
		.await
		.expect("Failed to build app");

	app.run().await.expect("Failed to run app");
}
