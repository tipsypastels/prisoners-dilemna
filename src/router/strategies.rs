use crate::{
    components::{layout::Layout, strategies::StrategiesList},
    id::Id,
    models::{Duel, NativeStrategy, Player, Strategy},
    router::Route,
    state::{use_state_context, Action},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn StrategiesPage() -> Html {
    let state = use_state_context();
    let navigator = use_navigator().unwrap();

    let strategies = use_memo(state.custom_strategies.clone(), |c| {
        NativeStrategy::ALL
            .into_iter()
            .map(Strategy::from)
            .chain(c.iter().map(Strategy::from))
            .collect::<Vec<_>>()
    });

    let selection1 = use_state(|| None::<usize>);
    let selection2 = use_state(|| None::<usize>);

    let s1 = selection1.and_then(|i| strategies.get(i)).cloned();
    let s2 = selection2.and_then(|i| strategies.get(i)).cloned();
    let can_submit = s1.is_some() && s2.is_some();

    let submit = Callback::from({
        let state = state.clone();
        let (s1, s2) = (s1.clone(), s2.clone());

        move |_| {
            let Some((s1, s2)) = s1.clone().zip(s2.clone()) else {
                return;
            };

            let (p1, p2) = (Player::new(s1), Player::new(s2));
            let duel = Duel::new(p1, p2, /* TODO */ 10);
            let id = Id::new();

            state.dispatch(Action::DuelNew(id, duel));
            navigator.push(&Route::Duel { id });
        }
    });

    let submit_button = html! {
        <button onclick={submit} disabled={!can_submit}>
            {"Submit"}
        </button>
    };

    html! {
        <Layout rcontrol={submit_button}>
            if let Some(selected_strategy1) = s1 {
                <div>{selected_strategy1.name()}</div>
            }
            if let Some(selected_strategy2) = s2 {
                <div>{selected_strategy2.name()}</div>
            }
            <StrategiesList
                strategies={strategies}
                selection1={selection1}
                selection2={selection2}
            />

        </Layout>
    }
}
