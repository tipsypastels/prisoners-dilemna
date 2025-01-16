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
        <div class="m-auto w-[800px] max-w-full pt-16">
            <header class="mb-4 flex border-b-4 border-b-lime-900 pb-4">
                <h1 class="grow text-center text-4xl font-bold text-lime-900">
                    {title}
                </h1>
            </header>

            <main>
                {props.children.clone()}
            </main>
        </div>
    }
}
