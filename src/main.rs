#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	use axum::Router;
	use hn_smart_aggregator::app::*;
	use hn_smart_aggregator::config::Config;
	use hn_smart_aggregator::services::aggregator;
	use hn_smart_aggregator::state::AppState;
	use leptos::config::get_configuration;
	use leptos::prelude::*;
	use leptos_axum::{generate_route_list, LeptosRoutes};
	use std::sync::Arc;

	// Initialize logging
	tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))).init();

	tracing::info!("Starting Smart HN Aggregator...");

	// Load configuration
	let config = Arc::new(Config::load().await?);
	tracing::info!("Configuration loaded");

	// Setup database
	let db_pool = sqlx::SqlitePool::connect(&config.database_url).await?;
	tracing::info!("Database connected");

	// Run migrations
	sqlx::migrate!("./migrations").run(&db_pool).await?;
	tracing::info!("Migrations completed");

	// Create app state
	let app_state = AppState { db_pool: db_pool.clone(), config: config.clone() };

	// Spawn background worker
	let worker_pool = db_pool.clone();
	let worker_config = config.clone();
	tokio::spawn(async move {
		tracing::info!("Starting background aggregator worker...");
		if let Err(e) = aggregator::run_aggregator_loop(worker_pool, worker_config).await {
			tracing::error!("Background worker failed: {}", e);
		}
	});

	// Setup Leptos
	let conf = get_configuration(None).unwrap();
	let addr = conf.leptos_options.site_addr;
	let leptos_options = conf.leptos_options;
	let routes = generate_route_list(App);

	// Build Axum router with context
	let app = Router::new()
		.leptos_routes_with_context(
			&leptos_options,
			routes,
			{
				let app_state = app_state.clone();
				move || provide_context(app_state.clone())
			},
			{
				let leptos_options = leptos_options.clone();
				move || shell(leptos_options.clone())
			},
		)
		.fallback(leptos_axum::file_and_error_handler(shell))
		.with_state(leptos_options);

	// Start server
	tracing::info!("Server listening on http://{}", &addr);
	let listener = tokio::net::TcpListener::bind(&addr).await?;
	axum::serve(listener, app.into_make_service()).await?;

	Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	// Client-side main
}
