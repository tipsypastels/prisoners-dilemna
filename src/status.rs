use crate::{Choice, History};

#[derive(Debug)]
pub struct Status {
    history: History,
    score: u32,
}

impl Status {
    pub(crate) fn new() -> Self {
        Self {
            history: History::new(),
            score: 0,
        }
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn add(&mut self, choice: Choice, score: u32) {
        self.history.add(choice);
        self.score += score;
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.score = 0;
    }
}
