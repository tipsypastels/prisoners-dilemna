use crate::{components::layout::Layout, id::Id, router::Route, state::use_state_context};
use yew::prelude::*;
use yew_router::components::Redirect;

#[derive(Debug, Properties, PartialEq)]
pub struct DuelPageProps {
    pub id: Id,
}

#[function_component]
pub fn DuelPage(props: &DuelPageProps) -> Html {
    let state = use_state_context();
    let id = props.id;
    let Some(duel) = state.duels.get(&id) else {
        return html! { <Redirect<Route> to={Route::Strategies} /> };
    };

    html! {
        <Layout>
            {"Duel"}
        </Layout>
    }
}
