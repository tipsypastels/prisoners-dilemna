use implicit_clone::ImplicitClone;

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq)]
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

    pub fn matrix(c1: Self, c2: Self) -> (u32, u32) {
        match (c1, c2) {
            (Self::Cooperate, Self::Cooperate) => (3, 3),
            (Self::Cooperate, Self::Defect) => (0, 5),
            (Self::Defect, Self::Cooperate) => (5, 0),
            (Self::Defect, Self::Defect) => (1, 1),
        }
    }
}
