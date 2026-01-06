use leptos::prelude::*;

#[component]
pub fn ThemeToggle(
	dark_mode: Signal<bool>,
	set_dark_mode: WriteSignal<bool>,
) -> impl IntoView {
	view! {
		<button
			class="theme-toggle"
			on:click=move |_| set_dark_mode.update(|dm| *dm = !*dm)
			aria-label="Toggle dark mode"
		>
			{move || if dark_mode.get() { "☀️" } else { "🌙" }}
		</button>
	}
}
