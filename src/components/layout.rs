use crate::router::Route;
use yew::prelude::*;
use yew_router::hooks::use_route;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}

#[function_component]
pub fn Layout(props: &LayoutProps) -> Html {
    let route = use_route::<Route>().unwrap();
    html! {
        <main>
            <h1>{route.title()}</h1>

            <div>
                {props.children.clone()}
            </div>
        </main>
    }
}
