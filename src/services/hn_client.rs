#[cfg(feature = "ssr")]
use anyhow::Result;

#[cfg(feature = "ssr")]
use crate::db::models::HnItem;

#[cfg(feature = "ssr")]
const HN_API_BASE: &str = "https://hacker-news.firebaseio.com/v0";

#[cfg(feature = "ssr")]
pub async fn fetch_top_stories(limit: usize) -> Result<Vec<i64>> {
	let url = format!("{}/topstories.json", HN_API_BASE);
	let client = reqwest::Client::new();

	let story_ids: Vec<i64> = client
		.get(&url)
		.send()
		.await?
		.json()
		.await?;

	Ok(story_ids.into_iter().take(limit).collect())
}

#[cfg(feature = "ssr")]
pub async fn fetch_item(id: i64) -> Result<HnItem> {
	let url = format!("{}/item/{}.json", HN_API_BASE, id);
	let client = reqwest::Client::new();

	let item = client
		.get(&url)
		.send()
		.await?
		.json()
		.await?;

	Ok(item)
}
