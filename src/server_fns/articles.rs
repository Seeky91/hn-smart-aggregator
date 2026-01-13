use crate::db::models::{Article, SortDirection, SortField};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::state::AppState;

#[server]
pub async fn get_interesting_articles(sort_field: SortField, sort_direction: SortDirection, category: String) -> Result<Vec<Article>, ServerFnError> {
	use crate::db::repository;
	let state = expect_context::<AppState>();

	repository::get_interesting_articles(&state.db_pool, sort_field, sort_direction, category)
		.await
		.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn get_categories() -> Result<Vec<String>, ServerFnError> {
	use crate::config::Config;
	let config = Config::load().await.map_err(|e| ServerFnError::new(e.to_string()))?;
	Ok(config.categories)
}
