use super::StrategiesEntry;
use crate::{
    models::{NativeStrategy, Strategy},
    state::use_state_context,
};
use yew::prelude::*;

pub type StrategiesListSelectionHandle = UseStateHandle<Option<usize>>;

#[derive(Debug, Properties, PartialEq)]
pub struct StrategiesListProps {
    pub selection1: StrategiesListSelectionHandle,
    pub selection2: StrategiesListSelectionHandle,
}

#[function_component]
pub fn StrategiesList(props: &StrategiesListProps) -> Html {
    let selection1 = props.selection1.clone();
    let selection2 = props.selection2.clone();

    let state = use_state_context();
    let strategies = use_memo(state.custom_strategies.clone(), |c| {
        NativeStrategy::ALL
            .into_iter()
            .map(Strategy::from)
            .chain(c.iter().map(Strategy::from))
            .enumerate()
            .collect::<Vec<_>>()
    });

    let entries = strategies
        .iter()
        .map(|(i, s)| {
            html! {
                <StrategiesEntry
                    index={i}
                    strategy={s.clone()}
                    selections={(selection1.clone(), selection2.clone())}
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
