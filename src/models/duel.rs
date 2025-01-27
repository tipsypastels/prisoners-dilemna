use super::{Player, PlayerStatus};
use crate::models::Choice;
use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, ImplicitClone, PartialEq)]
pub struct Duel {
    p1: Player,
    p2: Player,
    turns: DuelTurns,
}

impl Duel {
    pub fn new(p1: Player, p2: Player, turns: u32) -> Self {
        Self {
            p1,
            p2,
            turns: DuelTurns::new(turns),
        }
    }

    pub fn next(self) -> Self {
        let turns = self.turns.next();
        if turns == self.turns {
            return self;
        }

        let v1 = self.view(&self.p1, &self.p2);
        let v2 = self.view(&self.p2, &self.p1);

        let c1 = self.p1.strategy().turn(v1);
        let c2 = self.p2.strategy().turn(v2);

        let (s1, s2) = Choice::matrix(c1, c2);

        let p1 = self.p1.next(c1, s1);
        let p2 = self.p2.next(c2, s2);

        Self { p1, p2, turns }
    }

    pub fn p1(&self) -> &Player {
        &self.p1
    }

    pub fn p2(&self) -> &Player {
        &self.p2
    }

    pub fn turns(&self) -> DuelTurns {
        self.turns
    }

    pub fn outcome(&self) -> Option<DuelOutcome> {
        self.turns.is_done().then(|| {
            let s1 = self.p1.status().score();
            let s2 = self.p2.status().score();
            match () {
                () if s1 > s2 => DuelOutcome::Win(DuelPlayerId::P1),
                () if s2 > s1 => DuelOutcome::Win(DuelPlayerId::P2),
                () => DuelOutcome::Tie,
            }
        })
    }

    pub fn outcome_for(&self, player: DuelPlayerId) -> Option<DuelPlayerOutcome> {
        Some(match self.outcome()? {
            DuelOutcome::Tie => DuelPlayerOutcome::Tie,
            DuelOutcome::Win(p) if p == player => DuelPlayerOutcome::Won,
            DuelOutcome::Win(_) => DuelPlayerOutcome::Lost,
        })
    }

    fn view<'a>(&'a self, me: &'a Player, them: &'a Player) -> DuelView<'a> {
        DuelView::new(me.status(), them.status(), self.turns.cur)
    }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, ImplicitClone, PartialEq)]
pub struct DuelTurns {
    pub cur: u32,
    pub max: u32,
}

impl DuelTurns {
    fn new(max: u32) -> Self {
        Self { cur: 0, max }
    }

    fn next(self) -> Self {
        if self.is_done() {
            self
        } else {
            Self {
                cur: self.cur + 1,
                max: self.max,
            }
        }
    }

    pub fn is_done(self) -> bool {
        self.cur >= self.max
    }
}

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub struct DuelView<'a> {
    pub me: &'a PlayerStatus,
    pub them: &'a PlayerStatus,
    pub turn: u32,
}

impl<'a> DuelView<'a> {
    fn new(me: &'a PlayerStatus, them: &'a PlayerStatus, turn: u32) -> Self {
        Self { me, them, turn }
    }
}

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub enum DuelOutcome {
    Win(DuelPlayerId),
    Tie,
}

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub enum DuelPlayerId {
    P1,
    P2,
}

impl DuelPlayerId {
    pub fn is_p1(self) -> bool {
        matches!(self, Self::P1)
    }

    pub fn is_p2(self) -> bool {
        matches!(self, Self::P2)
    }
}

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
pub enum DuelPlayerOutcome {
    Won,
    Lost,
    Tie,
}

impl DuelPlayerOutcome {
    pub fn is_won(self) -> bool {
        matches!(self, Self::Won)
    }

    pub fn is_lost(self) -> bool {
        matches!(self, Self::Lost)
    }

    pub fn is_tie(self) -> bool {
        matches!(self, Self::Tie)
    }
}
