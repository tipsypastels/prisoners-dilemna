use rand::{rngs::ThreadRng, Rng};
use std::fmt;

/// Returns a `Choice::Cooperate` or `Choice::Defect` as determined
/// by the provided implementation of `rand::Rng`.
///
/// ```
/// use prisoners_dilemna::strategies::RANDOM_CHOICE;
/// # let my_rng_impl = rand::thread_rng();
/// let strategy = RANDOM_CHOICE(my_rng_impl);
/// ```
///
/// This type implements `Default`, which uses `rand::thread_rng()` as
/// the implementation.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct RANDOM_CHOICE<R: Rng + fmt::Debug + 'static>(pub R);

impl<R: Rng + fmt::Debug + 'static> RANDOM_CHOICE<R> {
    pub const fn new(rng: R) -> Self {
        Self(rng)
    }
}

impl<R: Rng + fmt::Debug + 'static> crate::CustomStrategy for RANDOM_CHOICE<R> {
    fn turn(&mut self, _view: crate::View) -> crate::Choice {
        self.0.gen()
    }
}

impl<R: Rng + fmt::Debug + 'static> fmt::Display for RANDOM_CHOICE<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Random Choice")
    }
}

impl Default for RANDOM_CHOICE<ThreadRng> {
    fn default() -> Self {
        Self(rand::thread_rng())
    }
}
