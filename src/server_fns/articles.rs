use crate::db::models::{Article, CategoryCount, SortDirection, SortField};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::state::AppState;

#[server]
pub async fn get_interesting_articles(sort_field: SortField, sort_direction: SortDirection, category: String) -> Result<Vec<Article>, ServerFnError> {
	use crate::db::repository;
	let state = expect_context::<AppState>();

	repository::get_interesting_articles(&state.db_pool, sort_field, sort_direction, category).await.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn get_categories_with_counts() -> Result<Vec<CategoryCount>, ServerFnError> {
	use crate::config::Config;
	use crate::state::AppState;
	use std::collections::HashMap;

	let state = expect_context::<AppState>();
	let config = Config::load().await.map_err(|e| ServerFnError::new(e.to_string()))?;

	let db_counts = sqlx::query!(
		r#"
		SELECT category as "category!", COUNT(*) as "count!"
		FROM articles
		WHERE is_interesting = 1 AND category IS NOT NULL AND category != ''
		GROUP BY category
		"#
	)
	.fetch_all(&state.db_pool)
	.await
	.map_err(|e| ServerFnError::new(e.to_string()))?;

	let counts_map: HashMap<String, i32> = db_counts.into_iter().map(|row| (row.category, row.count as i32)).collect();

	let mut final_categories: Vec<CategoryCount> = config
		.categories
		.into_iter()
		.map(|cat_name| CategoryCount { count: *counts_map.get(&cat_name).unwrap_or(&0), category: cat_name })
		.collect();

	final_categories.sort_by(|a, b| b.count.cmp(&a.count));

	Ok(final_categories)
}
