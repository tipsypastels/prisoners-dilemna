mod bindings;
mod components;
mod ext;
mod id;
mod models;
mod router;
mod state;

use self::{
    router::Router,
    state::{use_state_persistence, State, StateContextProvider},
};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let state = use_reducer(State::default);

    use_state_persistence(state.clone());

    html! {
        <StateContextProvider context={state}>
            <Router />
        </StateContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[macro_export]
macro_rules! tw {
    ($class:literal) => {
        $class
    };
}
