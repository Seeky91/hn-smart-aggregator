#[cfg(feature = "ssr")]
use anyhow::Result;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct Config {
	pub database_url: String,
	pub ollama_url: String,
	pub ollama_model: String,
	pub persona: String,
}

#[cfg(feature = "ssr")]
impl Config {
	pub async fn load() -> Result<Self> {
		// Load .env file
		dotenvy::dotenv().ok();

		// Load persona.txt from filesystem
		let persona = tokio::fs::read_to_string("persona.txt")
			.await
			.unwrap_or_else(|_| {
				tracing::warn!("persona.txt not found, using default");
				"You are a helpful AI assistant analyzing Hacker News articles.".to_string()
			});

		Ok(Self {
			database_url: std::env::var("DATABASE_URL")
				.unwrap_or_else(|_| "sqlite:articles.db".to_string()),
			ollama_url: std::env::var("OLLAMA_URL")
				.unwrap_or_else(|_| "http://localhost:11434".to_string()),
			ollama_model: std::env::var("OLLAMA_MODEL")
				.unwrap_or_else(|_| "qwen2.5:7b".to_string()),
			persona,
		})
	}
}
