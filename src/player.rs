use crate::{Choice, Status, Strategy, View};

#[derive(Debug)]
pub struct Player {
    strategy: Strategy,
    status: Status,
}

impl Player {
    pub fn new(strategy: impl Into<Strategy>) -> Self {
        Self {
            strategy: strategy.into(),
            status: Status::new(),
        }
    }

    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn turn(&mut self, them: &Self, turn: u32) -> Choice {
        let view = View::new(&self.status, &them.status, turn);
        self.strategy.turn(view)
    }

    pub fn add(&mut self, choice: Choice, score: u32) {
        self.status.add(choice, score);
    }

    pub fn clear(&mut self) {
        self.status.clear();
    }
}

impl<S> From<S> for Player
where
    S: Into<Strategy>,
{
    fn from(strategy: S) -> Self {
        Self::new(strategy)
    }
}
