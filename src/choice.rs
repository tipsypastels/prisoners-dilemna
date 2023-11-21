#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Choice {
    Cooperate,
    Defect,
}

impl Choice {
    pub fn is_cooperate(self) -> bool {
        matches!(self, Self::Cooperate)
    }

    pub fn is_defect(self) -> bool {
        matches!(self, Self::Defect)
    }

    pub fn matrix(choices: (Self, Self)) -> (u32, u32) {
        match choices {
            (Self::Cooperate, Self::Cooperate) => (3, 3),
            (Self::Cooperate, Self::Defect) => (0, 5),
            (Self::Defect, Self::Cooperate) => (5, 0),
            (Self::Defect, Self::Defect) => (1, 1),
        }
    }
}

#[cfg(feature = "rand")]
impl rand::distributions::Distribution<Choice> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        if rng.gen() {
            Choice::Cooperate
        } else {
            Choice::Defect
        }
    }
}
