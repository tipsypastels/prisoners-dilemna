use implicit_clone::ImplicitClone;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/js/core/dist/index.js")]
extern "C" {
    #[must_use]
    #[derive(Debug, Clone, ImplicitClone, PartialEq)]
    pub type EditorView;

    pub fn set_public_url(public_url: &str);
    pub async fn editor_init(parent: HtmlElement) -> EditorView;
}
