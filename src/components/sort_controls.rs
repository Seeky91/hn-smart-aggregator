use leptos::prelude::*;
use crate::db::models::{SortField, SortDirection};

#[component]
pub fn SortControls(
	sort_field: Signal<SortField>,
	set_sort_field: WriteSignal<SortField>,
	sort_direction: Signal<SortDirection>,
	set_sort_direction: WriteSignal<SortDirection>,
) -> impl IntoView {
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
				<option value="date" selected={move || sort_field.get() == SortField::Date}>
					"Date"
				</option>
				<option value="score" selected={move || sort_field.get() == SortField::Score}>
					"Score"
				</option>
				<option value="priority" selected={move || sort_field.get() == SortField::Priority}>
					"Priority"
				</option>
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
		</div>
	}
}
