use crate::{
    components::icon::Icon,
    models::{Choice, DuelTurns, PlayerHistoryEntry, PlayerStatus},
};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct DuelsPlayerChoicesProps {
    pub status: PlayerStatus,
    pub turns: DuelTurns,
}

#[function_component]
pub fn DuelsPlayerChoices(props: &DuelsPlayerChoicesProps) -> Html {
    let entries = (0..props.turns.max as usize).map(|i| props.status.history().get(i).copied());
    let cols = entries.map(|entry| html! { <Col entry={entry} /> });

    html! {
        <ol class="flex gap-2">
            {cols.collect::<Html>()}

            <ColBase
                icon={ColBaseIcon::None}
                score={props.status.score()}
            />
        </ol>
    }
}

#[derive(Debug, Properties, PartialEq)]
struct ColProps {
    entry: Option<PlayerHistoryEntry>,
}

#[function_component]
fn Col(props: &ColProps) -> Html {
    let Some(entry) = props.entry else {
        return html! { <ColBase /> };
    };

    let icon = match entry.choice {
        Choice::Cooperate => ColBaseIcon::Icon("hand", "Cooperate", "bg-green-600"),
        Choice::Defect => ColBaseIcon::Icon("hand-fist", "Defect", "bg-red-600"),
    };

    html! { <ColBase icon={icon} score={Some(entry.gain)} /> }
}

#[derive(Debug, Properties, PartialEq)]
struct ColBaseProps {
    #[prop_or_default]
    icon: ColBaseIcon,
    #[prop_or_default]
    score: Option<u32>,
}

#[derive(Debug, Default, PartialEq)]
enum ColBaseIcon {
    Icon(&'static str, &'static str, &'static str),
    #[default]
    Empty,
    None,
}

#[function_component]
fn ColBase(props: &ColBaseProps) -> Html {
    let class =
        "flex h-12 w-12 items-center justify-center border-2 border-b-0 border-slate-800 text-2xl text-white";

    html! {
        <li class="w-12">
            if let ColBaseIcon::Icon(name, label, iclass) = props.icon {
                <div class={classes!(iclass, class)} aria-label={label}>
                    <Icon name={name} />
                </div>
            } else if let ColBaseIcon::Empty = props.icon {
                <div class={class} />
            } else {
                <div class="h-12" />
            }

            <div class="flex h-12 w-12 items-center justify-center rounded-b-md border-2 border-slate-800 text-2xl">
                if let Some(score) = props.score {
                    {score}
                }
            </div>
        </li>
    }
}
