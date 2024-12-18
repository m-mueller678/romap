use crate::{Empty, RoMap};
use either::Either;

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, A: RoMap<'a, K, V>> RoMap<'a, K, V> for Option<A> {
    const ITER_ORDER_SORTED: bool = A::ITER_ORDER_SORTED;

    fn contains_key(self, k: &K) -> bool {
        option_to_either(self).contains_key(k)
    }

    fn get(self, k: &K) -> Option<&'a V> {
        option_to_either(self).get(k)
    }

    fn get_key(self, k: &K) -> Option<&'a K> {
        option_to_either(self).get_key(k)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        option_to_either(self).get_key_value(k)
    }

    fn is_empty(self) -> bool {
        option_to_either(self).is_empty()
    }

    fn len(self) -> usize {
        option_to_either(self).len()
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        option_to_either(self).keys()
    }

    fn values(self) -> impl 'a + Iterator<Item = &'a V> {
        option_to_either(self).values()
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        option_to_either(self).iter()
    }
}

fn option_to_either<A>(x: Option<A>) -> Either<A, Empty> {
    match x {
        Some(x) => Either::Left(x),
        None => Either::Right(Empty),
    }
}
