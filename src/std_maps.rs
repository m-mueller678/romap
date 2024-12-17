#![cfg(feature = "std")]

use crate::RoMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

impl<'a, K: Hash + Eq + 'a, V: 'a> RoMap<'a, K, V> for &'a HashMap<K, V> {
    fn contains_key(self, k: &K) -> bool {
        HashMap::contains_key(self, k)
    }

    fn get(self, k: &K) -> Option<&'a V> {
        HashMap::get(self, k)
    }

    fn is_empty(self) -> bool {
        HashMap::is_empty(self)
    }

    fn len(self) -> usize {
        HashMap::len(self)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        HashMap::get_key_value(self, k)
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
impl<'a, K: Hash + Eq + 'a> RoMap<'a, K, ()> for &'a HashSet<K> {
    fn contains_key(self, k: &K) -> bool {
        HashSet::contains(self, k)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a ())> {
        Some((HashSet::get(self, k)?, &()))
    }

    fn is_empty(self) -> bool {
        HashSet::is_empty(self)
    }

    fn len(self) -> usize {
        HashSet::len(self)
    }

    fn keys(self) -> impl Iterator<Item = &'a K> {
        HashSet::iter(self)
    }

    fn iter(self) -> impl Iterator<Item = (&'a K, &'a ())> {
        HashSet::iter(self).map(|x| (x, &()))
    }
}

impl<'a, K: Ord + 'a, V: 'a> RoMap<'a, K, V> for &'a BTreeMap<K, V> {
    const ITER_ORDER_SORTED: bool = true;

    fn contains_key(self, k: &K) -> bool {
        BTreeMap::contains_key(self, k)
    }

    fn get(self, k: &K) -> Option<&'a V> {
        BTreeMap::get(self, k)
    }

    fn is_empty(self) -> bool {
        BTreeMap::is_empty(self)
    }

    fn len(self) -> usize {
        BTreeMap::len(self)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)> {
        BTreeMap::get_key_value(self, k)
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

impl<'a, K: Ord + 'a> RoMap<'a, K, ()> for &'a BTreeSet<K> {
    fn contains_key(self, k: &K) -> bool {
        BTreeSet::contains(self, k)
    }

    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a ())> {
        Some((BTreeSet::get(self, k)?, &()))
    }

    fn is_empty(self) -> bool {
        BTreeSet::is_empty(self)
    }

    fn len(self) -> usize {
        BTreeSet::len(self)
    }

    fn keys(self) -> impl Iterator<Item = &'a K> {
        BTreeSet::iter(self)
    }

    fn iter(self) -> impl Iterator<Item = (&'a K, &'a ())> {
        BTreeSet::iter(self).map(|x| (x, &()))
    }
}
