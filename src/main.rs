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
    yew::Renderer::<App>::new().render();
}

#[macro_export]
macro_rules! tw {
    ($class:literal) => {
        $class
    };
}
