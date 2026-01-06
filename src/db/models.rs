// Article database model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Article {
	pub id: i64,
	pub hn_id: i64,
	pub title: String,
	pub url: Option<String>,
	pub score: i64,
	pub timestamp: i64,
	pub fetched_at: String,
	pub ai_analysis_done: bool,
	pub is_interesting: bool,
	pub reason: Option<String>,
	pub priority: Option<i64>,
}

// HN API response models (server-side only)
#[cfg(feature = "ssr")]
#[derive(Debug, serde::Deserialize)]
pub struct HnItem {
	pub id: i64,
	#[serde(default)]
	pub title: Option<String>,
	#[serde(default)]
	pub url: Option<String>,
	#[serde(default)]
	pub score: Option<i64>,
	pub time: i64,
}

// Ollama analysis result (server-side only)
#[cfg(feature = "ssr")]
#[derive(Debug, serde::Deserialize)]
pub struct AnalysisResult {
	pub relevant: bool,
	pub reason: String,
	pub priority: i64,
}

// Sorting enums (shared between client and server)
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SortField {
	Date,
	Score,
	Priority,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SortDirection {
	Ascending,
	Descending,
}

impl Default for SortField {
	fn default() -> Self {
		Self::Date
	}
}

impl Default for SortDirection {
	fn default() -> Self {
		Self::Descending
	}
}
