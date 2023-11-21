use crate::{Choice, SimpleStrategy};

macro_rules! simple_strategies {
    ($(#[name = $name:literal] $(#[$meta:meta])* $vis:vis fn $ident:ident($view:pat) $body:block)*) => {
        $(
            $(#[$meta])*
            $vis const $ident: SimpleStrategy = SimpleStrategy::new($name, |$view| $body);
        )*
    };
}

simple_strategies! {
    #[name = "Always Cooperate"]
    /// Always returns `Choice::Cooperate`.
    pub fn ALWAYS_COOPERATE(_) {
        Choice::Cooperate
    }

    #[name = "Always Defect"]
    /// Always returns `Choice::Defect`.
    pub fn ALWAYS_DEFECT(_) {
        Choice::Defect
    }

    #[name = "Grudger"]
    /// Returns `Choice::Defect` if the opposition has *ever* done the same,
    /// otherwise returns `Choice::Cooperate`.
    pub fn GRUDGER(view) {
        if view.them().history().ever_defected() {
            Choice::Defect
        } else {
            Choice::Cooperate
        }
    }

    #[name = "Pavlov"]
    /// Returns `Choice::Cooperate` if its latest choice was the same as
    /// the opposition's latest choice. If not, or if this is the first turn,
    /// return `Choice::Defect`.
    pub fn PAVLOV(view) {
        match view.me().history().latest().zip(view.them().history().latest()) {
            Some((a, b)) if a == b => Choice::Cooperate,
            _ => Choice::Defect,
        }
    }

    #[name = "Suspicious Tit For Tat"]
    /// Returns the opposition's previous choice, or `Choice::Defect`
    /// if this is the first turn.
    pub fn SUSPICIOUS_TIT_FOR_TAT(view) {
        view.them().history().latest().unwrap_or(Choice::Defect)
    }

    #[name = "Tit For Tat"]
    /// Returns the opposition's previous choice, or `Choice::Cooperate`
    /// if this is the first turn.
    pub fn TIT_FOR_TAT(view) {
        view.them().history().latest().unwrap_or(Choice::Cooperate)
    }

    #[name = "Tit For Two Tats"]
    /// Returns `Choice::Cooperate` unless the opposition's previous two choices
    /// have both been `Choice::Defect` , in which case it returns `Choice::Defect`.
    pub fn TIT_FOR_TWO_TATS(view) {
        if view.them().history().latest_n(2).all(Choice::is_defect) {
            Choice::Defect
        } else {
            Choice::Cooperate
        }
    }
}
