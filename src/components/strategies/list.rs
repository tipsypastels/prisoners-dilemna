use super::StrategiesEntry;
use crate::models::Strategy;
use std::rc::Rc;
use yew::prelude::*;

pub type StrategiesListSelectionHandle = UseStateHandle<Option<usize>>;

#[derive(Debug, Properties, PartialEq)]
pub struct StrategiesListProps {
    pub strategies: Rc<Vec<Strategy>>,
    pub selection1: StrategiesListSelectionHandle,
    pub selection2: StrategiesListSelectionHandle,
}

#[function_component]
pub fn StrategiesList(props: &StrategiesListProps) -> Html {
    let strategies = props.strategies.clone();
    let selection1 = props.selection1.clone();
    let selection2 = props.selection2.clone();

    let entries = strategies
        .iter()
        .enumerate()
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
