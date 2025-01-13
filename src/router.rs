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
    #[cfg(feature = "hash-router")]
    type MyRouter = HashRouter;
    #[cfg(not(feature = "hash-router"))]
    type MyRouter = BrowserRouter;

    html! {
        <div>
            <MyRouter basename={option_env!("PUBLIC_URL")}>
                <Link<Route> to={Route::Index}>{"Go to Index"}</Link<Route>>
                <Link<Route> to={Route::Other}>{"Go to Other"}</Link<Route>>
                <Switch<Route> render={switch} />
            </MyRouter>
        </div>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Index => html! { <div class="font-bold">{"Index"}</div> },
        Route::Other => html! { <div class="text-red-600">{"Other"}</div> },
    }
}
