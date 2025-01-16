use crate::models::{Choice, DuelView};
use Choice::*;

pub fn always_cooperate(_view: DuelView) -> Choice {
    Cooperate
}

pub fn always_defect(_view: DuelView) -> Choice {
    Defect
}

pub fn tit_for_tat(view: DuelView) -> Choice {
    view.them.history().last_choice().unwrap_or(Cooperate)
}
