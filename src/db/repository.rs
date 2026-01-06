#[cfg(feature = "ssr")]
use anyhow::Result;

#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

#[cfg(feature = "ssr")]
use crate::db::models::{Article, HnItem, AnalysisResult, SortField, SortDirection};

#[cfg(feature = "ssr")]
pub async fn upsert_article(pool: &SqlitePool, item: &HnItem) -> Result<()> {
	let title = item.title.clone().unwrap_or_default();
	let score = item.score.unwrap_or(0);

	sqlx::query(
		r#"
		INSERT INTO articles (hn_id, title, url, score, timestamp)
		VALUES (?, ?, ?, ?, ?)
		ON CONFLICT(hn_id) DO UPDATE SET
			score = excluded.score,
			title = excluded.title
		"#
	)
	.bind(item.id)
	.bind(title)
	.bind(&item.url)
	.bind(score)
	.bind(item.time)
	.execute(pool)
	.await?;

	Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_unanalyzed_articles(pool: &SqlitePool) -> Result<Vec<Article>> {
	let articles = sqlx::query_as::<_, Article>(
		r#"
		SELECT id, hn_id, title, url, score, timestamp, fetched_at,
		       ai_analysis_done, is_interesting, reason, priority
		FROM articles
		WHERE ai_analysis_done = 0
		ORDER BY fetched_at DESC
		"#
	)
	.fetch_all(pool)
	.await?;

	Ok(articles)
}

#[cfg(feature = "ssr")]
pub async fn update_analysis(
	pool: &SqlitePool,
	article_id: i64,
	analysis: AnalysisResult,
) -> Result<()> {
	sqlx::query(
		r#"
		UPDATE articles
		SET ai_analysis_done = 1,
		    is_interesting = ?,
		    reason = ?,
		    priority = ?
		WHERE id = ?
		"#
	)
	.bind(analysis.relevant)
	.bind(analysis.reason)
	.bind(analysis.priority)
	.bind(article_id)
	.execute(pool)
	.await?;

	Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_interesting_articles(
	pool: &SqlitePool,
	sort_field: SortField,
	sort_direction: SortDirection,
) -> Result<Vec<Article>> {
	// Build ORDER BY clause dynamically
	let order_by = match sort_field {
		SortField::Date => match sort_direction {
			SortDirection::Descending => "fetched_at DESC",
			SortDirection::Ascending => "fetched_at ASC",
		},
		SortField::Score => match sort_direction {
			SortDirection::Descending => "score DESC",
			SortDirection::Ascending => "score ASC",
		},
		SortField::Priority => match sort_direction {
			SortDirection::Descending => "priority DESC NULLS LAST, fetched_at DESC",
			SortDirection::Ascending => "priority ASC NULLS LAST, fetched_at ASC",
		},
	};

	let query = format!(
		r#"
		SELECT id, hn_id, title, url, score, timestamp, fetched_at,
		       ai_analysis_done, is_interesting, reason, priority
		FROM articles
		WHERE is_interesting = 1
		ORDER BY {}
		LIMIT 50
		"#,
		order_by
	);

	let articles = sqlx::query_as::<_, Article>(&query)
		.fetch_all(pool)
		.await?;

	Ok(articles)
}
