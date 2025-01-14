use crate::bindings;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
pub fn Editor() -> Html {
    let node_ref = use_node_ref();
    let view_ref = use_mut_ref(|| None::<bindings::EditorView>);

    use_effect_with((node_ref.clone(), view_ref), |(node_ref, view_ref)| {
        if view_ref.borrow().is_some() {
            return;
        }

        let node_ref = node_ref.clone();
        let view_ref = view_ref.clone();

        spawn_local(async move {
            let parent = node_ref.cast::<HtmlElement>().unwrap();
            let view = bindings::editor_init(parent).await;
            view_ref.borrow_mut().replace(view);
        });
    });

    html! {
        <div ref={node_ref} />
    }
}
