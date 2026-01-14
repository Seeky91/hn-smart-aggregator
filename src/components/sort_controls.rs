use crate::db::models::{SortDirection, SortField};
use crate::server_fns::articles::get_categories_with_counts;
use leptos::prelude::*;

#[component]
pub fn SortControls(
	sort_field: Signal<SortField>,
	set_sort_field: WriteSignal<SortField>,
	sort_direction: Signal<SortDirection>,
	set_sort_direction: WriteSignal<SortDirection>,
	selected_category: Signal<String>,
	set_selected_category: WriteSignal<String>,
) -> impl IntoView {
	let categories_resource = Resource::new(|| (), |_| get_categories_with_counts());

	view! {
		<div class="sort-controls">
			<label for="sort-field">"Sort by:"</label>
			<select
				id="sort-field"
				class="sort-select"
				on:change=move |ev| {
					let value = event_target_value(&ev);
					let field = match value.as_str() {
						"score" => SortField::Score,
						"priority" => SortField::Priority,
						_ => SortField::Date,
					};
					set_sort_field.set(field);
				}
			>
				<option value="date" selected={move || sort_field.get() == SortField::Date}>"Date"</option>
				<option value="score" selected={move || sort_field.get() == SortField::Score}>"Score"</option>
				<option value="priority" selected={move || sort_field.get() == SortField::Priority}>"Priority"</option>
			</select>

			<select
				class="sort-select"
				on:change=move |ev| {
					let value = event_target_value(&ev);
					let direction = match value.as_str() {
						"asc" => SortDirection::Ascending,
						_ => SortDirection::Descending,
					};
					set_sort_direction.set(direction);
				}
			>
				<option value="desc" selected={move || sort_direction.get() == SortDirection::Descending}>
					{move || match sort_field.get() {
						SortField::Date => "Newest first",
						SortField::Score => "Highest first",
						SortField::Priority => "Highest first",
					}}
				</option>
				<option value="asc" selected={move || sort_direction.get() == SortDirection::Ascending}>
					{move || match sort_field.get() {
						SortField::Date => "Oldest first",
						SortField::Score => "Lowest first",
						SortField::Priority => "Lowest first",
					}}
				</option>
			</select>

			<select
				id="category-field"
				class="sort-select"
				on:change=move |ev| {set_selected_category.set(event_target_value(&ev));}
			>
				<option value="" selected=move || selected_category.get().is_empty()>"All categories"</option>
				<Suspense fallback=|| view! { <option>"Loading…"</option> }>
					{move || Suspend::new(async move {
						let res = categories_resource.await;
						match res {
							Ok(cats) => {
								cats.into_iter()
									.map(|cat| {
										let cat_for_attr = cat.clone();
										let cat_for_logic = cat.clone();
										let is_selected = move || selected_category.get() == cat_for_logic.category;

										view! {
											<option disabled={cat.count == 0} value=cat_for_attr.category selected=is_selected>
												{format!("{} ({})", cat.category, cat.count)}
											</option>
										}
									})
									.collect_view()
									.into_any()
							}
							Err(_) => view! { <option>"Error"</option> }.into_any(),
						}
					})}
				</Suspense>
			</select>
		</div>
	}
}
