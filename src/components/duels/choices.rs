use crate::{
    components::icon::Icon,
    models::{Choice, DuelPlayerOutcome, DuelTurns, PlayerHistoryEntry, PlayerStatus},
    tw,
};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct DuelsPlayerChoicesProps {
    pub status: PlayerStatus,
    pub turns: DuelTurns,
    #[prop_or_default]
    pub outcome: Option<DuelPlayerOutcome>,
}

#[function_component]
pub fn DuelsPlayerChoices(props: &DuelsPlayerChoicesProps) -> Html {
    let entries = (0..props.turns.max as usize).map(|i| props.status.history().get(i).copied());
    let cols = entries.map(|entry| html! { <Col entry={entry} /> });

    let total_score_class = props.outcome.map(|outcome| match outcome {
        DuelPlayerOutcome::Won => tw!("bg-green-600"),
        DuelPlayerOutcome::Lost => tw!("bg-red-600"),
        DuelPlayerOutcome::Tie => tw!("text-red-600"),
    });

    html! {
        <ol class="flex gap-2">
            {cols.collect::<Html>()}

            <ColBase
                icon={ColBaseIcon::None}
                score={props.status.score()}
                score_class={total_score_class}
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
        Choice::Cooperate => ColBaseIcon::Icon("hand", "Cooperate", tw!("bg-green-600")),
        Choice::Defect => ColBaseIcon::Icon("hand-fist", "Defect", tw!("bg-red-600")),
    };

    html! { <ColBase icon={icon} score={Some(entry.gain)} /> }
}

#[derive(Debug, Properties, PartialEq)]
struct ColBaseProps {
    #[prop_or_default]
    icon: ColBaseIcon,
    #[prop_or_default]
    score: Option<u32>,
    #[prop_or_default]
    score_class: Option<&'static str>,
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
    let icon_base_class = tw!("flex h-12 w-12 items-center justify-center border-2 border-b-0 border-slate-800 text-2xl text-white");
    let score_base_class = tw!("flex h-12 w-12 items-center justify-center rounded-b-md border-2 border-slate-800 text-2xl");

    html! {
        <li class="w-12">
            if let ColBaseIcon::Icon(name, label, class) = props.icon {
                <div class={classes!(icon_base_class, class)} aria-label={label}>
                    <Icon name={name} />
                </div>
            } else if let ColBaseIcon::Empty = props.icon {
                <div class={icon_base_class} />
            } else {
                <div class="h-12" />
            }

            <div class={classes!(score_base_class, props.score_class)}>
                if let Some(score) = props.score {
                    {score}
                }
            </div>
        </li>
    }
}
