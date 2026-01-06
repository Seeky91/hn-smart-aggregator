#[cfg(feature = "ssr")]
use anyhow::{Context, Result};

#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::db::models::{AnalysisResult, Article};

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize)]
struct OllamaRequest {
	model: String,
	messages: Vec<Message>,
	stream: bool,
	format: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize)]
struct Message {
	role: String,
	content: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct OllamaResponse {
	message: MessageContent,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct MessageContent {
	content: String,
}

#[cfg(feature = "ssr")]
pub async fn analyze_article(persona: &str, article: &Article, ollama_url: &str, model: &str) -> Result<AnalysisResult> {
	let prompt = format!(
		"Analyze this Hacker News article and respond in JSON format with: {{\"relevant\": boolean, \"reason\": \"explanation\", \"priority\": number (1-5)}}\n\nPersona: {}\n\nArticle Title: {}\nURL: {}",
		persona,
		article.title,
		article.url.as_deref().unwrap_or("N/A")
	);

	let request = OllamaRequest { model: model.to_string(), messages: vec![Message { role: "user".to_string(), content: prompt }], stream: false, format: "json".to_string() };

	let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;

	let response = client
		.post(format!("{}/api/chat", ollama_url))
		.json(&request)
		.send()
		.await
		.context("Failed to send request to Ollama")?
		.json::<OllamaResponse>()
		.await
		.context("Failed to parse Ollama response")?;

	let analysis: AnalysisResult = serde_json::from_str(&response.message.content).context("Failed to parse analysis JSON")?;

	Ok(analysis)
}
