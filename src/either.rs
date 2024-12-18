use crate::RoMap;
use either::Either;

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, A: RoMap<'a, K, V>, B: RoMap<'a, K, V>> RoMap<'a, K, V>
    for Either<A, B>
{
    const ITER_ORDER_SORTED: bool = A::ITER_ORDER_SORTED && B::ITER_ORDER_SORTED;

    fn contains_key(self, k: &K) -> bool {
        match self {
            Either::Left(a) => a.contains_key(k),
            Either::Right(b) => b.contains_key(k),
        }
    }

    fn get(self, k: &K) -> Option<&'a V> {
        match self {
            Either::Left(a) => a.get(k),
            Either::Right(b) => b.get(k),
        }
    }

    fn get_key(self, k: &K) -> Option<&'a K> {
        match self {
            Either::Left(a) => a.get_key(k),
            Either::Right(b) => b.get_key(k),
        }
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        match self {
            Either::Left(a) => a.get_key_value(k),
            Either::Right(b) => b.get_key_value(k),
        }
    }

    fn is_empty(self) -> bool {
        match self {
            Either::Left(a) => a.is_empty(),
            Either::Right(b) => b.is_empty(),
        }
    }

    fn len(self) -> usize {
        match self {
            Either::Left(a) => a.len(),
            Either::Right(b) => b.len(),
        }
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        match self {
            Either::Left(a) => Either::Left(a.keys()),
            Either::Right(b) => Either::Right(b.keys()),
        }
        .into_iter()
    }

    fn values(self) -> impl 'a + Iterator<Item = &'a V> {
        match self {
            Either::Left(a) => Either::Left(a.values()),
            Either::Right(b) => Either::Right(b.values()),
        }
        .into_iter()
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        match self {
            Either::Left(a) => Either::Left(a.iter()),
            Either::Right(b) => Either::Right(b.iter()),
        }
        .into_iter()
    }
}
