use crate::RoMap;
use core::marker::PhantomData;

/// An Adapter that discards values, returning only keys.
/// ```
/// # use std::collections::{HashSet};
/// # use romap::{RoMap, RoMapSet};
/// fn to_set<'a, K: 'a, V: 'a>(x: impl RoMap<'a, K, V>) -> impl RoMap<'a, K, ()> {
///     RoMapSet::from(x)
/// }
/// ```
pub struct RoMapSet<'a, K, V, M> {
    inner: M,
    _p: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K: 'a, V: 'a, M: RoMap<'a, K, V>> From<M> for RoMapSet<'a, K, V, M> {
    fn from(value: M) -> Self {
        RoMapSet {
            inner: value,
            _p: PhantomData,
        }
    }
}

impl<'a, K: 'a, V: 'a, M: RoMap<'a, K, V>> Copy for RoMapSet<'a, K, V, M> {}

impl<'a, K: 'a, V: 'a, M: RoMap<'a, K, V>> Clone for RoMapSet<'a, K, V, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, K: 'a, V: 'a, M: RoMap<'a, K, V>> RoMap<'a, K, ()> for RoMapSet<'a, K, V, M> {
    const ITER_ORDER_SORTED: bool = M::ITER_ORDER_SORTED;

    fn contains_key(self, k: &K) -> bool {
        self.inner.contains_key(k)
    }

    fn get(self, k: &K) -> Option<&'a ()> {
        if self.inner.contains_key(k) {
            Some(&())
        } else {
            None
        }
    }

    fn is_empty(self) -> bool {
        self.inner.is_empty()
    }

    fn len(self) -> usize {
        self.inner.len()
    }

    fn get_key(self, k: &K) -> Option<&'a K> {
        self.inner.get_key(k)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a ())> {
        Some((self.inner.get_key(k)?, &()))
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        self.inner.keys()
    }

    fn values(self) -> impl 'a + Iterator<Item = &'a ()> {
        core::iter::repeat(&()).take(self.inner.len())
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a ())> {
        self.inner.keys().map(|k| (k, &()))
    }
}
