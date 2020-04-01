#![recursion_limit = "256"]

use vywrs::Vywrs;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

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

pub enum VywrsMessage {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
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

mod listing;
mod neq_assign;

mod components;
mod services;
mod vywrs;

#[wasm_bindgen]
pub fn entrypoint() -> Result<(), JsValue> {
    yew::start_app::<Vywrs>();
    Ok(())
}
