use crate::RoMap;
use std::borrow::Borrow;
use std::marker::PhantomData;

pub trait KeyMapper<K, K2>: Copy {
    const ORDER_PRESERVING: bool;
    fn map_key_in<R>(self, k: &K2, f: impl FnOnce(&K) -> R) -> R;
    fn map_key_out(self, k: &K) -> &K2;
}

#[derive(Clone, Copy)]
pub struct UnRef;

impl<'a, K> KeyMapper<&'a K, K> for UnRef {
    const ORDER_PRESERVING: bool = true;

    fn map_key_in<R>(self, k: &K, f: impl FnOnce(&&'a K) -> R) -> R {
        f(k)
    }

    fn map_key_out(self, k: &&K) -> &'a K {
        k
    }
}

pub fn map_keys<'a, K: 'a, K2: 'a, V: 'a>(
    map: impl RoMap<'a, K, V>,
    km: impl KeyMapper<K, K2> + 'a,
) -> impl RoMap<'a, K2, V> {
    KeyMappedRoMap {
        inner: map,
        km,
        _p: PhantomData,
    }
}

struct KeyMappedRoMap<KM, M, K> {
    inner: M,
    km: KM,
    _p: PhantomData<fn() -> K>,
}

impl<KM: Copy, M: Copy, K> Clone for KeyMappedRoMap<KM, M, K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<KM: Copy, M: Copy, K> Copy for KeyMappedRoMap<KM, M, K> {}

impl<'a, K: 'a, K2: 'a, V: 'a, KM: KeyMapper<K, K2> + 'a, M: RoMap<'a, K, V>> RoMap<'a, K2, V>
    for KeyMappedRoMap<KM, M, K>
{
    const ITER_ORDER_SORTED: bool = M::ITER_ORDER_SORTED && KM::ORDER_PRESERVING;

    fn contains_key(self, k: &K2) -> bool {
        self.km.map_key_in(k, |k| self.inner.contains_key(k))
    }

    fn get(self, k: &K2) -> Option<&'a V> {
        self.km.map_key_in(k, |k| self.inner.get(k))
    }

    fn get_key(self, k: &K2) -> Option<&'a K2> {
        let k = self.km.map_key_in(k, |k| self.inner.get_key(k))?;
        Some(self.km.map_key_out(k))
    }

    fn get_key_value(self, k: &K2) -> Option<(&'a K2, &'a V)> {
        let (k, v) = self.km.map_key_in(k, |k| self.inner.get_key_value(k))?;
        Some((self.km.map_key_out(k), v))
    }

    fn is_empty(self) -> bool {
        self.inner.is_empty()
    }

    fn len(self) -> usize {
        self.inner.len()
    }

    fn keys(self) -> impl Iterator<Item = &'a K2> {
        self.inner.keys().map(move |k| self.km.map_key_out(k))
    }

    fn values(self) -> impl Iterator<Item = &'a V> {
        self.inner.values()
    }

    fn iter(self) -> impl Iterator<Item = (&'a K2, &'a V)> {
        self.inner
            .iter()
            .map(move |(k, v)| (self.km.map_key_out(k), v))
    }
}
