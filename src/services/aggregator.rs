#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use anyhow::Result;

#[cfg(feature = "ssr")]
use tokio::time::{interval, Duration};

#[cfg(feature = "ssr")]
use crate::config::Config;

#[cfg(feature = "ssr")]
use crate::db::repository;

#[cfg(feature = "ssr")]
use crate::services::{hn_client, ollama_client};

#[cfg(feature = "ssr")]
pub async fn run_aggregator_loop(db_pool: sqlx::SqlitePool, config: Arc<Config>) -> Result<()> {
	// Run immediately on startup
	tracing::info!("Running initial aggregator cycle...");
	if let Err(e) = fetch_and_analyze_cycle(&db_pool, &config).await {
		tracing::error!("Initial aggregator cycle failed: {}", e);
	}

	// Then run every N minutes (from config)
	let mut interval = interval(Duration::from_secs(config.fetch_interval_minutes * 60));
	interval.tick().await; // Skip first tick (already ran above)

	loop {
		interval.tick().await;

		tracing::info!("Starting aggregator cycle...");
		if let Err(e) = fetch_and_analyze_cycle(&db_pool, &config).await {
			tracing::error!("Aggregator cycle failed: {}", e);
		}
	}
}

#[cfg(feature = "ssr")]
async fn fetch_and_analyze_cycle(db_pool: &sqlx::SqlitePool, config: &Config) -> Result<()> {
	// Step 1: Fetch top N HN stories (from config)
	tracing::info!("Fetching top {} HN stories...", config.top_stories_count);
	let story_ids = hn_client::fetch_top_stories(config.top_stories_count).await?;
	tracing::info!("Fetched {} story IDs", story_ids.len());

	// Step 2: Get details and save to database
	for id in story_ids {
		// Small delay to avoid rate limiting
		tokio::time::sleep(Duration::from_millis(100)).await;

		match hn_client::fetch_item(id).await {
			Ok(item) => {
				if let Err(e) = repository::upsert_article(db_pool, &item).await {
					tracing::warn!("Failed to save article {}: {}", id, e);
				}
			}
			Err(e) => {
				tracing::warn!("Failed to fetch item {}: {}", id, e);
			}
		}
	}

	// Step 3: Get unanalyzed articles
	let articles = repository::get_unanalyzed_articles(db_pool).await?;
	tracing::info!("Found {} unanalyzed articles", articles.len());

	// Step 4: Analyze with Ollama (sequential to avoid overwhelming local Ollama)
	for article in articles {
		match ollama_client::analyze_article(&config.persona, &article, &config.categories, &config.ollama_url, &config.ollama_model).await {
			Ok(mut analysis) => {
				// Validate category - if not in list, force to "Other"
				if !config.categories.iter().any(|c| c.eq_ignore_ascii_case(&analysis.category)) {
					tracing::warn!(
						"LLM returned invalid category '{}' for article '{}', using 'Other'",
						analysis.category,
						article.title
					);
					analysis.category = "Other".to_string();
				}

				tracing::info!(
					"Article '{}' analyzed: relevant={}, priority={}, category={}",
					article.title,
					analysis.relevant,
					analysis.priority,
					analysis.category
				);

				if let Err(e) = repository::update_analysis(db_pool, article.id, analysis).await {
					tracing::error!("Failed to save analysis for article {}: {}", article.id, e);
				}
			}
			Err(e) => {
				tracing::warn!("Failed to analyze article {}: {}", article.hn_id, e);
			}
		}

		// Small delay between analyses
		tokio::time::sleep(Duration::from_millis(500)).await;
	}

	tracing::info!("Aggregator cycle completed");
	Ok(())
}
