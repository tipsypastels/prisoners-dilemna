use implicit_clone::{
    unsync::{IArray, IMap},
    ImplicitClone,
};
use std::{hash::Hash, iter};

pub trait IArrayExt<T> {
    fn push(self, value: T) -> Self;
}

impl<T: ImplicitClone> IArrayExt<T> for IArray<T>
where
    T: ImplicitClone,
{
    fn push(self, value: T) -> Self {
        self.iter().chain(iter::once(value)).collect()
    }
}

pub trait IMapExt<K, V> {
    fn insert(self, key: K, value: V) -> Self;
}

impl<K, V> IMapExt<K, V> for IMap<K, V>
where
    K: Eq + Hash + ImplicitClone,
    V: PartialEq + ImplicitClone,
{
    fn insert(self, key: K, value: V) -> Self {
        self.iter().chain(iter::once((key, value))).collect()
    }
}
