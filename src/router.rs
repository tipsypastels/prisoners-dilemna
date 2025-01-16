mod duel;
mod strategies;

use crate::id::Id;
use implicit_clone::ImplicitClone;
use yew::prelude::*;
use yew_router::prelude::*;

pub type RouteLink = Link<Route>;

#[derive(Debug, Clone, ImplicitClone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Strategies,
    #[at("/game/:id")]
    Duel { id: Id },
}

impl Route {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Strategies => "Strategies",
            Self::Duel { .. } => "Game",
        }
    }
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
                <Switch<Route> render={switch} />
            </MyRouter>
        </div>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Strategies => html! { <strategies::StrategiesPage /> },
        Route::Duel { .. } => html! { <duel::DuelPage /> },
    }
}
