use implicit_clone::ImplicitClone;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, ImplicitClone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/other")]
    Other,
}

#[function_component]
pub fn Router() -> Html {
    gloo::console::log!(option_env!("GH_PAGES_BASENAME"));
    html! {
        <div>
            <BrowserRouter basename={option_env!("GH_PAGES_BASENAME")}>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Index => html! { <div class="font-bold">{"Index"}</div> },
        Route::Other => html! { <div class="text-red-600">{"Other"}</div> },
    }
}
