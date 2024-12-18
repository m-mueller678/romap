use crate::RoMap;

pub fn union<A, B>(a: A, b: B) -> Union<A, B> {
    Union { a, b }
}

#[derive(Copy, Clone)]
pub struct Union<A, B> {
    a: A,
    b: B,
}

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized, A: RoMap<'a, K, V>, B: RoMap<'a, K, V>> RoMap<'a, K, V>
    for Union<A, B>
{
    const ITER_ORDER_SORTED: bool = false;

    fn contains_key(self, k: &K) -> bool {
        self.a.contains_key(k) || self.b.contains_key(k)
    }

    fn get(self, k: &K) -> Option<&'a V> {
        self.a.get(k).or_else(|| self.b.get(k))
    }

    fn get_key(self, k: &K) -> Option<&'a K> {
        self.a.get_key(k).or_else(|| self.b.get_key(k))
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        self.a.get_key_value(k).or_else(|| self.b.get_key_value(k))
    }

    fn is_empty(self) -> bool {
        self.a.is_empty() && self.b.is_empty()
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        self.a
            .keys()
            .chain(self.b.keys().filter(move |k| !self.a.contains_key(k)))
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        self.a
            .iter()
            .chain(self.b.iter().filter(move |(k, _)| !self.a.contains_key(k)))
    }
}
