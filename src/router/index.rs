use crate::{
    layout::Layout,
    models::{Duel, NativeStrategy, Player, Strategy},
    state::{use_state_context, Action},
};
use yew::prelude::*;

#[function_component]
pub fn Index() -> Html {
    let state = use_state_context();

    let duel_new = {
        let state = state.clone();
        Callback::from(move |_| {
            let s1 = Strategy::Native(NativeStrategy::TIT_FOR_TAT);
            let s2 = Strategy::Native(NativeStrategy::ALWAYS_DEFECT);

            let p1 = Player::new(s1);
            let p2 = Player::new(s2);

            let duel = Duel::new(p1, p2, 10);

            state.dispatch(Action::Duel(duel));
        })
    };

    let duel_turn = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::DuelTurn);
        })
    };

    let duel_close = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::DuelClose);
        })
    };

    html! {
        <Layout>
            if let Some(duel) = &state.duel {
                {format!("{duel:?}")}

                <button onclick={duel_turn}>
                    {"Advance"}
                </button>

                <button onclick={duel_close}>
                    {"Close"}
                </button>
            } else {
                <button onclick={duel_new}>
                    {"New Duel"}
                </button>
            }
        </Layout>
    }
}
