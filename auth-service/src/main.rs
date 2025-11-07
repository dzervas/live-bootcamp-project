use auth_service::{prod, AppState, Application};

#[tokio::main]
async fn main() {
	let db_pool = configure_db_pool().await;

	let app_state = AppState::default();

	let app = Application::build(app_state, prod::APP_ADDRESS)
		.await
		.expect("Failed to build app");

	app.run().await.expect("Failed to run app");
}


async fn configure_db_pool() -> auth_service::DatabasePool {
	let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let db = auth_service::get_sql_pool(&url).await;
	sqlx::migrate!().run(&db).await.expect("Failed to run migrations");

	db
}
