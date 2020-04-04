#![recursion_limit = "256"]

mod components;
mod listing;
mod neq_assign;
mod services;
mod vywrs;

use components::Vywrs;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn entrypoint() -> Result<(), JsValue> {
    yew::start_app::<Vywrs>();
    Ok(())
}
