mod router;

use self::router::Router;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="p-4 px-8 font-bold">
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>

            <Router />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
