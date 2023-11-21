use crate::{choice::Choice, view::View};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SimpleStrategy {
    name: &'static str,
    turn: fn(view: View) -> Choice,
}

impl SimpleStrategy {
    pub const fn new(name: &'static str, turn: fn(view: View) -> Choice) -> Self {
        Self { name, turn }
    }
}

impl fmt::Debug for SimpleStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for SimpleStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

pub trait CustomStrategy: fmt::Display + fmt::Debug + 'static {
    fn turn(&mut self, view: View) -> Choice;
}

pub enum Strategy {
    Simple(SimpleStrategy),
    Custom(Box<dyn CustomStrategy>),
}

impl Strategy {
    pub fn turn(&mut self, view: View) -> Choice {
        match self {
            Self::Simple(s) => (s.turn)(view),
            Self::Custom(s) => s.turn(view),
        }
    }
}

impl fmt::Debug for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(s) => write!(f, "{s:?}"),
            Self::Custom(s) => write!(f, "{s:?}"),
        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(s) => write!(f, "{s}"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl From<SimpleStrategy> for Strategy {
    fn from(strategy: SimpleStrategy) -> Self {
        Self::Simple(strategy)
    }
}

impl<S> From<S> for Strategy
where
    S: CustomStrategy,
{
    fn from(strategy: S) -> Self {
        Self::Custom(Box::new(strategy))
    }
}
