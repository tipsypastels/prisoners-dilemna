use super::{Choice, Strategy};
use crate::ext::IArrayExt;
use implicit_clone::{unsync::IArray, ImplicitClone};
use std::ops::Deref;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Player {
    strategy: Strategy,
    status: PlayerStatus,
}

impl Player {
    pub fn new(strategy: Strategy) -> Self {
        Self {
            strategy,
            status: PlayerStatus::new(),
        }
    }

    pub fn next(self, choice: Choice, gain: u32) -> Self {
        Self {
            strategy: self.strategy,
            status: self.status.next(choice, gain),
        }
    }

    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    pub fn status(&self) -> &PlayerStatus {
        &self.status
    }
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct PlayerStatus {
    history: PlayerHistory,
    score: u32,
}

impl PlayerStatus {
    fn new() -> Self {
        Self {
            history: PlayerHistory::new(),
            score: 0,
        }
    }

    fn next(self, choice: Choice, gain: u32) -> Self {
        Self {
            history: self.history.next(choice, gain),
            score: self.score + gain,
        }
    }

    pub fn history(&self) -> &PlayerHistory {
        &self.history
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct PlayerHistory {
    items: IArray<PlayerHistoryItem>,
    ever_cooperated: bool,
    ever_defected: bool,
}

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub struct PlayerHistoryItem {
    pub choice: Choice,
    pub gain: u32,
}

impl PlayerHistory {
    fn new() -> Self {
        Self {
            items: IArray::default(),
            ever_cooperated: false,
            ever_defected: false,
        }
    }

    fn next(self, choice: Choice, gain: u32) -> Self {
        Self {
            items: self.items.push(PlayerHistoryItem { choice, gain }),
            ever_cooperated: self.ever_cooperated || choice.is_cooperate(),
            ever_defected: self.ever_defected || choice.is_defect(),
        }
    }

    pub fn first_choice(&self) -> Option<Choice> {
        self.items.first().copied().map(|i| i.choice)
    }

    pub fn last_choice(&self) -> Option<Choice> {
        self.items.last().copied().map(|i| i.choice)
    }

    pub fn ever_cooperated(&self) -> bool {
        self.ever_cooperated
    }

    pub fn ever_defected(&self) -> bool {
        self.ever_defected
    }
}

impl Deref for PlayerHistory {
    type Target = [PlayerHistoryItem];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
