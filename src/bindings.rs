use implicit_clone::ImplicitClone;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/js/core/dist/index.js")]
extern "C" {
    #[must_use]
    #[derive(Debug, Clone, ImplicitClone, PartialEq)]
    pub type EditorView;

    #[wasm_bindgen(js_name = getNextId)]
    pub fn get_next_id() -> u32;

    #[wasm_bindgen(js_name = setPublicUrl)]
    pub fn set_public_url(public_url: &str);

    #[wasm_bindgen(js_name = createEditor)]
    pub async fn create_editor(doc: &str, parent: HtmlElement) -> EditorView;
}
