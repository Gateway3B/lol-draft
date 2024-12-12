pub mod draft;
pub mod app;
pub mod entity;
pub mod api;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
