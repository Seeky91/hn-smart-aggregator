pub mod app;
pub mod components;
pub mod db;

#[cfg(feature = "ssr")]
pub mod config;

#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "ssr")]
pub mod services;

pub mod server_fns;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use leptos::mount::hydrate_body;
	console_error_panic_hook::set_once();
	hydrate_body(app::App);
}
