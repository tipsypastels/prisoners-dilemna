use super::choices::DuelsPlayerChoices;
use crate::models::{DuelTurns, Player};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct DuelsPlayerProps {
    pub player: Player,
    pub turns: DuelTurns,
}

#[function_component]
pub fn DuelsPlayer(props: &DuelsPlayerProps) -> Html {
    let (player, &turns) = (&props.player, &props.turns);
    html! {
        <section>
            <h3 class="text-3xl font-bold mb-4">
                {player.strategy().name()}
            </h3>

            <DuelsPlayerChoices
                status={player.status().clone()}
                turns={turns}
            />
        </section>
    }
}
