use super::StrategiesListSelectionHandle;
use crate::{models::Strategy, tw};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct StrategiesEntryProps {
    pub index: usize,
    pub strategy: Strategy,
    #[prop_or_default]
    pub selections: Option<(StrategiesListSelectionHandle, StrategiesListSelectionHandle)>,
}

#[function_component]
pub fn StrategiesEntry(props: &StrategiesEntryProps) -> Html {
    let index = props.index;
    let strategy = &props.strategy;

    html! {
        <li class="mb-4 p-4 shadow-lg">
            <div class="flex rounded-lg border-2 border-dashed border-lime-500 p-4">
                <div class="grow">
                    <h2 class="mb-2 text-xl font-bold">
                        {strategy.name()}
                    </h2>

                    <div class="text-gray-600">
                        {strategy.desc()}
                    </div>
                </div>

                if let Some((s1, s2)) = &props.selections {
                    <div class="flex gap-2">
                        <SelectButton
                            label={"Player 1"}
                            class="border-teal-700 text-teal-700 disabled:bg-teal-700"
                            index={index}
                            selection={s1.clone()}
                        />

                        <SelectButton
                            label={"Player 2"}
                            class="border-indigo-700 text-indigo-700 disabled:bg-indigo-700"
                            index={index}
                            selection={s2.clone()}
                        />
                    </div>
                }
            </div>
        </li>
    }
}

#[derive(Debug, Properties, PartialEq)]
struct SelectButtonProps {
    label: &'static str,
    class: &'static str,
    index: usize,
    selection: StrategiesListSelectionHandle,
}

#[function_component]
fn SelectButton(props: &SelectButtonProps) -> Html {
    let base_class = tw!("border-2 border-dashed font-bold px-6 rounded-lg disabled:border-solid disabled:text-white");
    let index = props.index;
    let selection = props.selection.clone();
    let selected = selection.is_some_and(|i| i == index);
    let onclick = move |_| selection.set(Some(index));

    html! {
        // TODO: Use `tailwind-merge`.
        <button class={classes!(props.class, base_class)} onclick={onclick} disabled={selected}>
            {props.label}
        </button>
    }
}
