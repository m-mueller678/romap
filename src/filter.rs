use crate::RoMap;

/// Filter depending on both key and value
pub fn filter_kv<
    'a,
    K: 'a + ?Sized,
    V: 'a + ?Sized,
    M: RoMap<'a, K, V>,
    F: Fn(&'a K, &'a V) -> bool + Copy + 'a,
>(
    map: M,
    filter: F,
) -> FilterKv<M, F> {
    FilterKv { m: map, f: filter }
}

/// See [filter_kv]
#[derive(Clone, Copy)]
pub struct FilterKv<M, F> {
    m: M,
    f: F,
}

impl<
        'a,
        K: 'a + ?Sized,
        V: 'a + ?Sized,
        M: RoMap<'a, K, V>,
        F: Fn(&'a K, &'a V) -> bool + Copy + 'a,
    > RoMap<'a, K, V> for FilterKv<M, F>
{
    const ITER_ORDER_SORTED: bool = M::ITER_ORDER_SORTED;

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        self.m.get_key_value(k).filter(|(k, v)| (self.f)(k, v))
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        self.m.iter().filter(move |(k, v)| (self.f)(k, v))
    }
}

/// Filter depending on key
///
/// For lookups, this consults the filter first and only accesses the map if the key passes the filter.
/// If the filter is expensive to compute, consider using [filter_kv] instead.
pub fn filter_key<
    'a,
    K: 'a + ?Sized,
    V: 'a + ?Sized,
    M: RoMap<'a, K, V>,
    F: Fn(&'a K) -> bool + Copy + 'a,
>(
    map: M,
    filter: F,
) -> FilterKey<M, F> {
    FilterKey { m: map, f: filter }
}

/// See [filter_key]
#[derive(Clone, Copy)]
pub struct FilterKey<M, F> {
    m: M,
    f: F,
}

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, M: RoMap<'a, K, V>, F: Fn(&K) -> bool + Copy + 'a>
    RoMap<'a, K, V> for FilterKey<M, F>
{
    const ITER_ORDER_SORTED: bool = M::ITER_ORDER_SORTED;

    fn contains_key(self, k: &K) -> bool {
        (self.f)(k) && self.m.contains_key(k)
    }

    fn get(self, k: &K) -> Option<&'a V> {
        if (self.f)(k) {
            self.m.get(k)
        } else {
            None
        }
    }

    fn get_key(self, k: &K) -> Option<&'a K> {
        if (self.f)(k) {
            self.m.get_key(k)
        } else {
            None
        }
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        if (self.f)(k) {
            self.m.get_key_value(k)
        } else {
            None
        }
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        self.m.keys().filter(move |k| (self.f)(k))
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        self.m.iter().filter(move |x| (self.f)(x.0))
    }
}
