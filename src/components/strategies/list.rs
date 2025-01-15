use super::StrategiesEntry;
use crate::{
    models::{NativeStrategy, Strategy},
    state::use_state_context,
};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct StrategiesListProps {}

#[function_component]
pub fn StrategiesList(props: &StrategiesListProps) -> Html {
    let state = use_state_context();
    let strategies = use_memo(state.custom_strategies.clone(), |c| {
        NativeStrategy::ALL
            .into_iter()
            .map(Strategy::from)
            .chain(c.iter().map(Strategy::from))
            .collect::<Vec<_>>()
    });

    let entries = strategies
        .iter()
        .map(|s| {
            html! {
                <StrategiesEntry
                    strategy={s.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
        <ul>
            {entries}
        </ul>
    }
}
