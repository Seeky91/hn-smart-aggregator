#[cfg(feature = "ssr")]
use anyhow::Result;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct Config {
	pub database_url: String,
	pub ollama_url: String,
	pub ollama_model: String,
	pub persona: String,
	pub categories: Vec<String>,
	pub fetch_interval_minutes: u64,
	pub top_stories_count: usize,
}

#[cfg(feature = "ssr")]
impl Config {
	pub async fn load() -> Result<Self> {
		dotenvy::dotenv().ok();

		let persona = match tokio::fs::read_to_string("persona.txt").await {
			Ok(content) => content,
			Err(_) => match tokio::fs::read_to_string("config/persona.txt").await {
				Ok(content) => content,
				Err(_) => {
					tracing::warn!("persona.txt not found (tried ./persona.txt and ./config/persona.txt), using default");
					"You are a helpful AI assistant analyzing Hacker News articles.".to_string()
				}
			}
		};

		let categories_text = match tokio::fs::read_to_string("categories.txt").await {
			Ok(content) => content,
			Err(_) => match tokio::fs::read_to_string("config/categories.txt").await {
				Ok(content) => content,
				Err(_) => {
					tracing::warn!("categories.txt not found (tried ./categories.txt and ./config/categories.txt), using default categories");
					"Programming\nWeb Development\nAI & Machine Learning\nOther".to_string()
				}
			}
		};

		let mut categories: Vec<String> = categories_text
			.lines()
			.map(|s| s.trim().to_string())
			.filter(|s| !s.is_empty())
			.collect();

		// Ensure "Other" is always available as fallback
		if !categories.iter().any(|c| c.eq_ignore_ascii_case("other")) {
			categories.push("Other".to_string());
		}

		Ok(Self {
			database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:articles.db".to_string()),
			ollama_url: std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".to_string()),
			ollama_model: std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5:7b".to_string()),
			persona,
			categories,
			fetch_interval_minutes: std::env::var("FETCH_INTERVAL_MINUTES")
				.ok()
				.and_then(|s| s.parse().ok())
				.unwrap_or(60),
			top_stories_count: std::env::var("TOP_STORIES_COUNT")
				.ok()
				.and_then(|s| s.parse().ok())
				.unwrap_or(15),
		})
	}
}
