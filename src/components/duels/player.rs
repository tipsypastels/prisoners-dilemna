use super::choices::DuelsPlayerChoices;
use crate::models::{DuelPlayerOutcome, DuelTurns, Player};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct DuelsPlayerProps {
    pub player: Player,
    pub turns: DuelTurns,
    #[prop_or_default]
    pub outcome: Option<DuelPlayerOutcome>,
}

#[function_component]
pub fn DuelsPlayer(props: &DuelsPlayerProps) -> Html {
    let (player, turns, outcome) = (&props.player, props.turns, &props.outcome);
    html! {
        <section>
            <h3 class="text-3xl font-bold mb-4">
                {player.strategy().name()}
            </h3>

            <DuelsPlayerChoices
                status={player.status().clone()}
                turns={turns}
                outcome={outcome.as_ref().cloned()}
            />
        </section>
    }
}
