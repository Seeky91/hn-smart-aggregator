use crate::db::models::Article;
use leptos::prelude::*;

#[component]
pub fn ArticleCard(article: Article) -> impl IntoView {
	view! {
		<div class="article-card">
			<h3>
				<a href={article.url.clone().unwrap_or_default()} target="_blank" rel="noopener noreferrer">
					{article.title.clone()}
				</a>
			</h3>
			<div class="meta">
				<span class="score">"⬆ " {article.score} " points"</span>
				{article.priority.map(|p| view! {
					<span class="priority">"Priority: " {p} "/5"</span>
				})}
				<span>{article.category}</span>
			</div>
			{article.reason.as_ref().map(|reason| {
				let r = reason.clone();
				view! { <p class="reason">{r}</p> }
			})}
		</div>
	}
}
