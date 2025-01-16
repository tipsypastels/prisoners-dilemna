mod native_impl;

use super::{Choice, DuelView};
use enum_assoc::Assoc;
use implicit_clone::{unsync::IString, ImplicitClone};

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum Strategy {
    Native(NativeStrategy),
    Custom(CustomStrategy),
}

impl Strategy {
    pub fn name(&self) -> &str {
        match self {
            Self::Native(s) => s.name(),
            Self::Custom(s) => &s.name,
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            Self::Native(s) => s.desc(),
            Self::Custom(s) => &s.desc,
        }
    }

    pub(super) fn turn(&self, view: DuelView) -> Choice {
        match self {
            Self::Native(s) => (s.turn())(view),
            Self::Custom(_) => todo!(),
        }
    }
}

impl From<NativeStrategy> for Strategy {
    fn from(strategy: NativeStrategy) -> Self {
        Self::Native(strategy)
    }
}

impl From<CustomStrategy> for Strategy {
    fn from(strategy: CustomStrategy) -> Self {
        Self::Custom(strategy)
    }
}

#[derive(Assoc, Debug, Copy, Clone, ImplicitClone, PartialEq)]
#[func(pub const fn name(self) -> &'static str)]
#[func(pub const fn desc(self) -> &'static str)]
#[func(const fn turn(self) -> fn(DuelView) -> Choice)]
pub enum NativeStrategy {
    #[assoc(
        name = "Always Cooperate", 
        desc = "Always chooses to cooperate.",
        turn = native_impl::always_cooperate,
    )]
    AlwaysCooperate,
    #[assoc(
        name = "Always Defect", 
        desc = "Always chooses to defect.", 
        turn = native_impl::always_defect,
    )]
    AlwaysDefect,
    #[assoc(
        name = "Tit-For-Tat",
        desc = "Starts with cooperate, then copies the foe's last move.",
        turn = native_impl::tit_for_tat,
    )]
    TitForTat,
}

impl NativeStrategy {
    pub const ALL: [Self; 3] = [Self::AlwaysCooperate, Self::AlwaysDefect, Self::TitForTat];
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct CustomStrategy {
    name: IString,
    desc: IString,
    turn: IString,
}

impl CustomStrategy {
    pub fn new(name: IString, desc: IString, turn: IString) -> Self {
        Self { name, desc, turn }
    }

    pub fn name(&self) -> &IString {
        &self.name
    }

    pub fn desc(&self) -> &IString {
        &self.desc
    }
}
