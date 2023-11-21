use crate::Status;

#[derive(Debug)]
pub struct View<'a> {
    me: &'a Status,
    them: &'a Status,
    turn: u32,
}

impl<'a> View<'a> {
    pub(crate) fn new(me: &'a Status, them: &'a Status, turn: u32) -> Self {
        Self { me, them, turn }
    }

    pub fn me(&self) -> &Status {
        self.me
    }

    pub fn them(&self) -> &Status {
        self.them
    }

    pub fn turn(&self) -> u32 {
        self.turn
    }
}
