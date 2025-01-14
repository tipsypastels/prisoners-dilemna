use super::{Choice, DuelView};
use implicit_clone::ImplicitClone;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum Strategy {
    Native(NativeStrategy),
}

impl Strategy {
    pub fn name(&self) -> &str {
        match self {
            Self::Native(n) => n.name,
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            Self::Native(n) => n.desc,
        }
    }

    pub(super) fn turn(&self, view: DuelView) -> Choice {
        match self {
            Self::Native(s) => (s.turn)(view),
        }
    }
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct NativeStrategy {
    name: &'static str,
    desc: &'static str,
    turn: fn(DuelView) -> Choice,
}

#[allow(unused)]
impl NativeStrategy {
    pub const ALWAYS_COOPERATE: Self = Self {
        name: "Always Cooperate",
        desc: "TODO",
        turn: |_| Choice::Cooperate,
    };

    pub const ALWAYS_DEFECT: Self = Self {
        name: "Always Defect",
        desc: "TODO",
        turn: |_| Choice::Defect,
    };

    pub const TIT_FOR_TAT: Self = Self {
        name: "Tit-For-Tat",
        desc: "TODO",
        turn: |view| {
            view.them
                .history()
                .last_choice()
                .unwrap_or(Choice::Cooperate)
        },
    };
}
