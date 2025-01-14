use implicit_clone::{unsync::IArray, ImplicitClone};
use std::iter;

pub trait IArrayExt<T> {
    fn push(self, value: T) -> Self;
}

impl<T: ImplicitClone> IArrayExt<T> for IArray<T> {
    fn push(self, value: T) -> Self {
        self.iter().chain(iter::once(value)).collect()
    }
}
