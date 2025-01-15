use crate::models::Strategy;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct StrategiesEntryProps {
    pub strategy: Strategy,
    #[prop_or_default]
    pub is_selected1: bool,
    #[prop_or_default]
    pub is_selected2: bool,
    #[prop_or_default]
    pub onselect: Option<Callback<()>>,
}

#[function_component]
pub fn StrategiesEntry(props: &StrategiesEntryProps) -> Html {
    let strategy = &props.strategy;
    html! {
        <li>
            <div>
                {strategy.name()}
            </div>
        </li>
    }
}
