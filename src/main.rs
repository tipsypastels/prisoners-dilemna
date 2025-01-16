mod bindings;
mod components;
mod ext;
mod id;
mod models;
mod router;
mod state;

use self::{
    router::Router,
    state::{State, StateContextProvider},
};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let state = use_reducer(State::default);
    let debug_state = format!("{state:?}");

    html! {
        <StateContextProvider context={state}>
            <pre><code>{debug_state}</code></pre>
            <Router />
        </StateContextProvider>
    }
}

fn main() {
    bindings::set_public_url(option_env!("PUBLIC_URL").unwrap_or_default());
    yew::Renderer::<App>::new().render();
}

#[macro_export]
macro_rules! tw {
    ($class:literal) => {
        $class
    };
}
