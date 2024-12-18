use crate::RoMap;
use core::marker::PhantomData;
use core::ops::Deref;

/// Applies some function to the values of a map.
pub fn project_value<
    'a,
    K: 'a + ?Sized,
    VI: 'a + ?Sized,
    VO: 'a + ?Sized,
    M: RoMap<'a, K, VI>,
    F: Fn(&'a VI) -> &'a VO + Copy + 'a,
>(
    map: M,
    projection: F,
) -> impl RoMap<'a, K, VO> {
    ValueProjection {
        m: map,
        f: projection,
        _p: PhantomData,
    }
}

/// Calls [deref](core::ops::Deref) on the values of a map.
pub fn deref_value<'a, K: 'a + ?Sized, VI: 'a + Deref + ?Sized, M: RoMap<'a, K, VI>>(
    map: M,
) -> impl RoMap<'a, K, VI::Target> {
    project_value(map, |v| &**v)
}

struct ValueProjection<M, F, VI: ?Sized> {
    m: M,
    f: F,
    _p: PhantomData<fn() -> VI>,
}

impl<M: Copy, F: Copy, VI: ?Sized> Clone for ValueProjection<M, F, VI> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M: Copy, F: Copy, VI: ?Sized> Copy for ValueProjection<M, F, VI> {}

impl<
        'a,
        K: 'a + ?Sized,
        VI: 'a + ?Sized,
        VO: 'a + ?Sized,
        M: RoMap<'a, K, VI>,
        F: Fn(&'a VI) -> &'a VO + Copy + 'a,
    > RoMap<'a, K, VO> for ValueProjection<M, F, VI>
{
    const ITER_ORDER_SORTED: bool = M::ITER_ORDER_SORTED;

    fn contains_key(self, k: &K) -> bool {
        self.m.contains_key(k)
    }

    fn get(self, k: &K) -> Option<&'a VO> {
        Some((self.f)(self.m.get(k)?))
    }

    fn get_key(self, k: &K) -> Option<&'a K> {
        self.m.get_key(k)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a VO)> {
        let (k, v) = self.m.get_key_value(k)?;
        Some((k, (self.f)(v)))
    }

    fn is_empty(self) -> bool {
        self.m.is_empty()
    }

    fn len(self) -> usize {
        self.m.len()
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        self.m.keys()
    }

    fn values(self) -> impl 'a + Iterator<Item = &'a VO> {
        self.m.values().map(move |v| (self.f)(v))
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a VO)> {
        self.m.iter().map(move |(k, v)| (k, (self.f)(v)))
    }
}
