use leptos::prelude::*;
use crate::components::article_card::ArticleCard;
use crate::db::models::Article;

#[component]
pub fn ArticleList(articles: Vec<Article>) -> impl IntoView {
	if articles.is_empty() {
		view! {
			<div class="empty-state">
				<p>"No interesting articles yet. The background worker is fetching and analyzing articles..."</p>
			</div>
		}.into_any()
	} else {
		view! {
			<div class="article-list">
				{articles.into_iter()
					.map(|article| view! { <ArticleCard article=article /> })
					.collect_view()}
			</div>
		}.into_any()
	}
}
