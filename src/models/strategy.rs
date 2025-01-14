use super::{Choice, DuelView};
use implicit_clone::ImplicitClone;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum Strategy {
    Native(NativeStrategy),
}

impl Strategy {
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
