use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
	components::{Route, Router, Routes},
	StaticSegment,
};

use crate::components::{article_list::ArticleList, sort_controls::SortControls, theme_toggle::ThemeToggle};
use crate::db::models::{SortDirection, SortField};
use crate::server_fns::articles::{get_interesting_articles};

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8"/>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
				<AutoReload options=options.clone() />
				<HydrationScripts options/>
				<MetaTags/>
			</head>
			<body>
				<App/>
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	let (dark_mode, set_dark_mode) = signal(true); // Start with dark mode

	view! {
		<Stylesheet id="leptos" href="/pkg/hn-smart-aggregator.css"/>
		<Title text="Smart HN Aggregator"/>
		<Meta name="description" content="AI-powered Hacker News aggregator"/>

		<Router>
			<main class=move || if dark_mode.get() { "dark" } else { "" }>
				<Routes fallback=|| "Page not found.".into_view()>
					<Route path=StaticSegment("") view=move || view! {
						<HomePage dark_mode=dark_mode.into() set_dark_mode=set_dark_mode />
					}/>
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn HomePage(dark_mode: Signal<bool>, set_dark_mode: WriteSignal<bool>) -> impl IntoView {
	let (sort_field, set_sort_field) = signal(SortField::Date);
	let (sort_direction, set_sort_direction) = signal(SortDirection::Descending);
	let (selected_category, set_selected_category) = signal(String::new());

	let articles = Resource::new(move || (sort_field.get(), sort_direction.get(), selected_category.get()), |(field, direction, cat)| get_interesting_articles(field, direction, cat));

	view! {
		<div class="container">
			<header class="header">
				<h1>"Smart HN Aggregator"</h1>
				<ThemeToggle dark_mode=dark_mode set_dark_mode=set_dark_mode />
			</header>

			<SortControls
				sort_field=sort_field.into()
				set_sort_field=set_sort_field
				sort_direction=sort_direction.into()
				set_sort_direction=set_sort_direction
				selected_category=selected_category.into()
				set_selected_category=set_selected_category
			/>

			<Suspense fallback=|| view! { <div class="loading">"Loading articles…"</div> }>
				{move || {
					articles.get().map(|result| {
						match result {
							Ok(articles) => view! {
								<ArticleList articles=articles />
							}.into_any(),
							Err(e) => view! {
								<div class="error">
									<p>"Error loading articles: " {e.to_string()}</p>
								</div>
							}.into_any(),
						}
					})
				}}
			</Suspense>
		</div>
	}
}
