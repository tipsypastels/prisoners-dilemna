use crate::{Choice, Player, Status, Strategy};
use std::fmt;

#[derive(Debug)]
pub struct Duellist {
    player: Player,
    index: Index,
}

impl Duellist {
    pub(crate) fn new(player: Player, index: Index) -> Self {
        Self { player, index }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn strategy(&self) -> &Strategy {
        self.player.strategy()
    }

    pub fn status(&self) -> &Status {
        self.player.status()
    }

    pub fn index(&self) -> Index {
        self.index
    }

    pub fn turn(&mut self, them: &Self, turn: u32) -> Choice {
        self.player.turn(&them.player, turn)
    }

    pub fn add(&mut self, choice: Choice, score: u32) {
        self.player.add(choice, score)
    }

    pub fn clear(&mut self) {
        self.player.clear()
    }
}

impl fmt::Display for Duellist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.index, self.strategy())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Index {
    P1 = 1,
    P2 = 2,
}

impl Index {
    pub fn is_p1(self) -> bool {
        matches!(self, Self::P1)
    }

    pub fn is_p2(self) -> bool {
        matches!(self, Self::P2)
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {}", *self as u8)
    }
}
