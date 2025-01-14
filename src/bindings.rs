use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/core/dist/index.js")]
extern "C" {
    pub fn set_public_url(public_url: &str);
    pub fn test_hi();
}
