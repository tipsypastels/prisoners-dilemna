#[derive(Debug)]
pub struct Turns {
    cur: u32,
    max: u32,
}

impl Turns {
    pub(crate) fn new(max: u32) -> Self {
        Self { cur: 0, max }
    }

    pub fn cur(&self) -> u32 {
        self.cur
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn clear(&mut self) {
        self.cur = 0;
    }
}

impl From<u32> for Turns {
    fn from(max: u32) -> Self {
        Self::new(max)
    }
}

impl Iterator for Turns {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.max {
            None
        } else {
            let cur = self.cur;
            self.cur += 1;
            Some(cur)
        }
    }
}
