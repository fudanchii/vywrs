#![recursion_limit = "256"]

#[derive(Copy, Clone, PartialEq)]
pub enum VywrsMode {
    List,
    Tile,
}

#[derive(Copy, Clone, PartialEq)]
pub enum VywrsTheme {
    Dark,
    Light,
}

impl std::ops::Deref for VywrsTheme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            VywrsTheme::Dark => "dark",
            VywrsTheme::Light => "light",
        }
    }
}

mod components;
mod services;
mod vywrs;

use vywrs::Vywrs;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn entrypoint() -> Result<(), JsValue> {
    yew::start_app::<Vywrs>();
    Ok(())
}
