use crate::bindings;
use implicit_clone::ImplicitClone;
use std::{fmt, str::FromStr};

#[derive(Debug, Copy, Clone, ImplicitClone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

impl Id {
    pub fn new() -> Self {
        Self(bindings::get_next_id())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Id {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}
