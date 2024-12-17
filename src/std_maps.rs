use crate::RoMap;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

impl<'a, K: Hash + Eq + 'a, V: 'a> RoMap<'a, K, V> for &'a HashMap<K, V> {
    fn contains_key(self, j: &K) -> bool {
        HashMap::contains_key(self, j)
    }

    fn get(self, j: &K) -> Option<&'a V> {
        HashMap::get(self, j)
    }

    fn len(self) -> usize {
        HashMap::len(self)
    }

    fn get_key_value(self, j: &K) -> Option<(&'a K, &'a V)> {
        HashMap::get_key_value(self, j)
    }

    fn keys(self) -> impl Iterator<Item = &'a K> {
        HashMap::keys(self)
    }

    fn values(self) -> impl Iterator<Item = &'a V> {
        HashMap::values(self)
    }

    fn iter(self) -> impl Iterator<Item = (&'a K, &'a V)> {
        HashMap::iter(self)
    }
}

impl<'a, K: Ord + 'a, V: 'a> RoMap<'a, K, V> for &'a BTreeMap<K, V> {
    const ITER_ORDER_DETERMINISTIC: bool = true;
    const ITER_ORDER_SORTED: bool = true;

    fn contains_key(self, j: &K) -> bool {
        BTreeMap::contains_key(self, j)
    }

    fn get(self, j: &K) -> Option<&'a V> {
        BTreeMap::get(self, j)
    }

    fn len(self) -> usize {
        BTreeMap::len(self)
    }

    fn get_key_value(self, j: &K) -> Option<(&'a K, &'a V)> {
        BTreeMap::get_key_value(self, j)
    }

    fn keys(self) -> impl Iterator<Item = &'a K> {
        BTreeMap::keys(self)
    }

    fn values(self) -> impl Iterator<Item = &'a V> {
        BTreeMap::values(self)
    }

    fn iter(self) -> impl Iterator<Item = (&'a K, &'a V)> {
        BTreeMap::iter(self)
    }
}
