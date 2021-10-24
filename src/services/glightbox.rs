use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Glightbox;
    pub fn GLightbox() -> Glightbox;
}
