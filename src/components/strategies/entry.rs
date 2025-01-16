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
        <li class="mb-4 p-4 shadow-lg transition hover:-translate-y-1">
            <div class="flex rounded-lg border-2 border-dashed border-lime-500 p-4">
                <div class="grow">
                    <h2 class="mb-2 text-xl font-bold">
                        {strategy.name()}
                    </h2>

                    <div class="text-gray-600">
                        {strategy.desc()}
                    </div>
                </div>
            </div>
        </li>
    }
}
