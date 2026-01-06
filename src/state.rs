#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use crate::config::Config;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
	pub db_pool: sqlx::SqlitePool,
	pub config: Arc<Config>,
}
