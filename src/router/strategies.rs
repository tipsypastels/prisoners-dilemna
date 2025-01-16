use crate::{
    components::{layout::Layout, strategies::StrategiesList},
    models::{NativeStrategy, Strategy},
    state::use_state_context,
};
use yew::prelude::*;

#[function_component]
pub fn StrategiesPage() -> Html {
    let state = use_state_context();
    let strategies = use_memo(state.custom_strategies.clone(), |c| {
        NativeStrategy::ALL
            .into_iter()
            .map(Strategy::from)
            .chain(c.iter().map(Strategy::from))
            .collect::<Vec<_>>()
    });

    let selection1 = use_state(|| None::<usize>);
    let selection2 = use_state(|| None::<usize>);

    html! {
        <Layout>
            <StrategiesList
                strategies={strategies}
                selection1={selection1}
                selection2={selection2}
            />
        </Layout>
    }
}
