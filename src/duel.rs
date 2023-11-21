use crate::{Choice, Player, Turns};
use std::fmt;

mod duellist;
mod outcome;

pub use duellist::{Duellist, Index};
pub use outcome::Outcome;

pub struct Duel {
    p1: Duellist,
    p2: Duellist,
    turns: Turns,
}

impl Duel {
    pub fn new(p1: impl Into<Player>, p2: impl Into<Player>, turns: impl Into<Turns>) -> Self {
        Self {
            p1: Duellist::new(p1.into(), Index::P1),
            p2: Duellist::new(p2.into(), Index::P2),
            turns: turns.into(),
        }
    }

    pub fn run(&mut self) -> Outcome {
        for turn in &mut self.turns {
            let c1 = self.p1.turn(&self.p2, turn);
            let c2 = self.p2.turn(&self.p1, turn);

            let (s1, s2) = Choice::matrix((c1, c2));

            self.p1.add(c1, s1);
            self.p2.add(c2, s2);
        }

        Outcome::new(&mut self.p1, &mut self.p2)
    }

    pub fn clear(&mut self) {
        self.p1.clear();
        self.p2.clear();
        self.turns.clear();
    }
}

impl fmt::Debug for Duel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Duel")
            .field("p1", &self.p1)
            .field("p2", &self.p2)
            .field("turns", &self.turns)
            .finish()
    }
}
