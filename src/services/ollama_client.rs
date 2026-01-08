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
pub async fn analyze_article(persona: &str, article: &Article, categories: &[String], ollama_url: &str, model: &str) -> Result<AnalysisResult> {
	let categories_str = categories.join(", ");

	let prompt = format!(
		r#"Analyze this Hacker News article.
Assign the most specific category from the list below.
Use 'Other' ONLY for news that does not fit any other category.

Output Format (JSON):
{{"relevant": boolean, "reason": "explanation", "priority": number (1-5), "category": "category_name"}}

Persona: {}

Available Categories (Strict): {}

Article Title: {}
Article URL: {}"#,
		persona,
		categories_str,
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

	let content = &response.message.content;

	// Try to extract JSON from response (in case there's extra text)
	let json_str = extract_json(content);

	// Log for debugging if parsing fails
	let analysis: AnalysisResult = serde_json::from_str(&json_str).map_err(|e| {
		tracing::error!("Failed to parse JSON. Error: {}. Raw content: {}", e, content);
		anyhow::anyhow!("Failed to parse analysis JSON")
	})?;

	Ok(analysis)
}

#[cfg(feature = "ssr")]
fn extract_json(content: &str) -> String {
	// Try to find JSON object in the content
	if let Some(start) = content.find('{') {
		if let Some(end) = content.rfind('}') {
			if start < end {
				return content[start..=end].to_string();
			}
		}
	}
	// If no JSON found, return original content
	content.to_string()
}
