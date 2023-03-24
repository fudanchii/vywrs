use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Instance;

    pub fn GLightbox() -> Instance;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Instance);
}
