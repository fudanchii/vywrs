#![recursion_limit = "512"]

mod components;
mod listing;
mod neq_assign;
mod services;
mod vywrs;

use components::Vywrs;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn entrypoint() -> Result<(), JsValue> {
    yew::start_app::<Vywrs>();
    Ok(())
}
