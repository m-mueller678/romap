use crate::RoMap;
use core::marker::PhantomData;

/// Discard values, replacing them with `&()`.
pub fn discard_values<'a, K: 'a + ?Sized, V: 'a + ?Sized, M: RoMap<'a, K, V>>(
    map: M,
) -> DiscardValues<'a, K, V, M> {
    DiscardValues {
        inner: map,
        _p: PhantomData,
    }
}

/// See [discard_values]
pub struct DiscardValues<'a, K: ?Sized, V: ?Sized, M> {
    inner: M,
    _p: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, M: RoMap<'a, K, V>> Copy for DiscardValues<'a, K, V, M> {}

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, M: RoMap<'a, K, V>> Clone for DiscardValues<'a, K, V, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, M: RoMap<'a, K, V>> RoMap<'a, K, ()>
    for DiscardValues<'a, K, V, M>
{
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
        core::iter::repeat_n(&(), self.inner.len())
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a ())> {
        self.inner.keys().map(|k| (k, &()))
    }
}
