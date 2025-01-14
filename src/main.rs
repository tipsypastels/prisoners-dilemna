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
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    #[wasm_bindgen(module = "/js/core/dist/index.js")]
    extern "C" {
        pub fn test_hi(public_url: Option<&str>);
    }

    test_hi(option_env!("PUBLIC_URL"));

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
