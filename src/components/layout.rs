use crate::router::Route;
use implicit_clone::unsync::IString;
use yew::prelude::*;
use yew_router::hooks::use_route;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub title: Option<IString>,
    pub children: Html,
}

#[function_component]
pub fn Layout(props: &LayoutProps) -> Html {
    let route = use_route::<Route>().unwrap();
    let title = props.title.as_deref().unwrap_or(route.title());

    html! {
        <main>
            <h1>{title}</h1>

            <div>
                {props.children.clone()}
            </div>
        </main>
    }
}
