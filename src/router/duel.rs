use crate::{
    components::{duels::DuelsPlayer, layout::Layout},
    id::Id,
    models::DuelPlayerId,
    router::Route,
    state::use_state_context,
};
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
            <DuelsPlayer
                player={duel.p1().clone()}
                turns={duel.turns()}
                outcome={duel.outcome_for(DuelPlayerId::P1)}
            />

            <DuelsPlayer
                player={duel.p2().clone()}
                turns={duel.turns()}
                outcome={duel.outcome_for(DuelPlayerId::P2)}
            />
        </Layout>
    }
}
