#[cfg(feature = "rand")]
mod random;
mod simple;

#[cfg(feature = "rand")]
pub use random::*;
pub use simple::*;
