use crate::{
    components::{duels::DuelsPlayer, layout::Layout, strategies::StrategiesList},
    models::{Duel, DuelPlayerId, NativeStrategy, Player, Strategy},
    state::{use_state_context, Action},
};
use yew::prelude::*;

#[function_component]
pub fn StrategiesPage() -> Html {
    html! {
        <Layout>
            <StrategiesList />
        </Layout>
    }

    // let duel_new = {
    //     let state = state.clone();
    //     Callback::from(move |_| {
    //         let s1 = Strategy::Native(NativeStrategy::TIT_FOR_TAT);
    //         let s2 = Strategy::Native(NativeStrategy::ALWAYS_DEFECT);

    //         let p1 = Player::new(s1);
    //         let p2 = Player::new(s2);

    //         let duel = Duel::new(p1, p2, 10);

    //         state.dispatch(Action::Duel(duel));
    //     })
    // };

    // let duel_turn = {
    //     let state = state.clone();
    //     Callback::from(move |_| {
    //         state.dispatch(Action::DuelTurn);
    //     })
    // };

    // let duel_close = {
    //     let state = state.clone();
    //     Callback::from(move |_| {
    //         state.dispatch(Action::DuelClose);
    //     })
    // };

    // html! {
    //     <Layout>
    //         if let Some(duel) = &state.duel {
    //             <h2>
    //                 {"Duel"}
    //             </h2>

    //             <DuelsPlayer
    //                 player={duel.p1().clone()}
    //                 turns={duel.turns()}
    //                 outcome={duel.outcome_for(DuelPlayerId::P1)}
    //             />

    //             <DuelsPlayer
    //                 player={duel.p2().clone()}
    //                 turns={duel.turns()}
    //                 outcome={duel.outcome_for(DuelPlayerId::P2)}
    //             />

    //             <button onclick={duel_turn}>
    //                 {"Advance"}
    //             </button>

    //             <button onclick={duel_close}>
    //                 {"Close"}
    //             </button>
    //         } else {
    //             <StrategiesList />

    //             <button onclick={duel_new}>
    //                 {"New Duel"}
    //             </button>
    //         }
    //     </Layout>
    // }
}
