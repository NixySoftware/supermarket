pub mod app;
#[cfg(feature = "ssr")]
pub mod auth;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
#[cfg(feature = "ssr")]
pub mod entities;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
