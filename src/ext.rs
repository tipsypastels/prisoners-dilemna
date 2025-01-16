use implicit_clone::{
    unsync::{IArray, IMap},
    ImplicitClone,
};
use std::{borrow::Borrow, hash::Hash, iter};

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
    fn remove<Q>(self, key: &Q) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    fn transform<Q>(self, key: &Q, f: impl FnOnce(V) -> V) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
}

impl<K, V> IMapExt<K, V> for IMap<K, V>
where
    K: Eq + Hash + ImplicitClone,
    V: PartialEq + ImplicitClone,
{
    fn insert(self, key: K, value: V) -> Self {
        self.iter().chain(iter::once((key, value))).collect()
    }

    fn remove<Q>(self, key: &Q) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.iter().filter(|(k, _)| k.borrow() != key).collect()
    }

    fn transform<Q>(self, key: &Q, f: impl FnOnce(V) -> V) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut f = Some(f);
        self.iter()
            .map(|(k, v)| {
                if k.borrow() == key {
                    (k, f.take().unwrap()(v))
                } else {
                    (k, v)
                }
            })
            .collect()
    }
}
