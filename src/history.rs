use crate::Choice;
use std::ops;

#[derive(Debug)]
pub struct History {
    choices: Vec<Choice>,
    ever_cooperated: bool,
    ever_defected: bool,
}

impl History {
    pub(crate) fn new() -> Self {
        Self {
            choices: Vec::new(),
            ever_cooperated: false,
            ever_defected: false,
        }
    }

    pub fn first(&self) -> Option<Choice> {
        self.choices.first().copied()
    }

    pub fn first_n(&self, n: usize) -> impl Iterator<Item = Choice> + '_ {
        self.choices.iter().take(n).copied()
    }

    pub fn latest(&self) -> Option<Choice> {
        self.choices.last().copied()
    }

    pub fn latest_n(&self, n: usize) -> impl Iterator<Item = Choice> + '_ {
        self.choices.iter().rev().take(n).copied()
    }

    pub fn ever_cooperated(&self) -> bool {
        self.ever_cooperated
    }

    pub fn ever_defected(&self) -> bool {
        self.ever_defected
    }

    pub fn add(&mut self, choice: Choice) {
        self.choices.push(choice);

        if choice == Choice::Cooperate {
            self.ever_cooperated = true;
        } else {
            self.ever_defected = true;
        }
    }

    pub fn clear(&mut self) {
        self.choices.clear();
        self.ever_cooperated = false;
        self.ever_defected = false;
    }
}

impl AsRef<[Choice]> for History {
    fn as_ref(&self) -> &[Choice] {
        self
    }
}

impl ops::Deref for History {
    type Target = [Choice];

    fn deref(&self) -> &Self::Target {
        &self.choices
    }
}
