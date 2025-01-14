mod bindings;
mod components;
mod ext;
mod layout;
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
    bindings::test_hi();

    let state = use_reducer(State::default);

    html! {
        <div class="p-4 px-8 font-bold">
            <StateContextProvider context={state}>
                <Router />
            </StateContextProvider>
        </div>
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
