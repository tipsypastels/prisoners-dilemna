use super::{Choice, DuelView};
use implicit_clone::{unsync::IString, ImplicitClone};
use js_sys::Function;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub enum Strategy {
    Native(NativeStrategy),
    Custom(CustomStrategy),
}

impl Strategy {
    pub fn name(&self) -> &str {
        match self {
            Self::Native(s) => s.name,
            Self::Custom(s) => &s.name,
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            Self::Native(s) => s.desc,
            Self::Custom(s) => &s.desc,
        }
    }

    pub(super) fn turn(&self, view: DuelView) -> Choice {
        match self {
            Self::Native(s) => (s.turn)(view),
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

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub struct NativeStrategy {
    name: &'static str,
    desc: &'static str,
    turn: fn(DuelView) -> Choice,
}

#[allow(unused)]
impl NativeStrategy {
    pub const ALL: [Self; 3] = [
        Self::ALWAYS_COOPERATE,
        Self::ALWAYS_DEFECT,
        Self::TIT_FOR_TAT,
    ];

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

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct CustomStrategy {
    name: IString,
    desc: IString,
    turn: Function,
}

impl CustomStrategy {
    pub fn new(name: IString, desc: IString, turn: Function) -> Self {
        Self { name, desc, turn }
    }

    pub fn name(&self) -> &IString {
        &self.name
    }

    pub fn desc(&self) -> &IString {
        &self.desc
    }
}
