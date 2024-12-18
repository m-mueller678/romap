use crate::RoMap;
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

macro_rules! impl_map {
    ($Container:ty) => {
        fn contains_key(self, k: &KO) -> bool {
            <$Container>::contains_key(self, k)
        }

        fn get(self, k: &KO) -> Option<&'a V> {
            <$Container>::get(self, k)
        }

        fn get_key_value(self, k: &KO) -> Option<(&'a KO, &'a V)> {
            let (k, v) = <$Container>::get_key_value(self, k)?;
            Some((k.borrow(), v))
        }

        fn is_empty(self) -> bool {
            <$Container>::is_empty(self)
        }

        fn len(self) -> usize {
            <$Container>::len(self)
        }

        fn keys(self) -> impl Iterator<Item = &'a KO> {
            <$Container>::keys(self).map(Borrow::borrow)
        }

        fn values(self) -> impl Iterator<Item = &'a V> {
            <$Container>::values(self)
        }

        fn iter(self) -> impl Iterator<Item = (&'a KO, &'a V)> {
            <$Container>::iter(self).map(|(k, v)| (k.borrow(), v))
        }
    };
}

impl<'a, KI: Hash + Eq + Borrow<KO>, V: 'a, KO: Hash + Eq + ?Sized + 'a> RoMap<'a, KO, V>
    for &'a HashMap<KI, V>
{
    impl_map!(HashMap::<KI, V>);
}

impl<'a, KI: Ord + Borrow<KO>, V: 'a, KO: Ord + ?Sized + 'a> RoMap<'a, KO, V>
    for &'a BTreeMap<KI, V>
{
    const ITER_ORDER_SORTED: bool = true;

    impl_map!(BTreeMap::<KI, V>);
}

macro_rules! impl_set {
    ($Container:ty) => {
        fn contains_key(self, k: &KO) -> bool {
            <$Container>::contains(self, k)
        }

        fn get_key_value(self, k: &KO) -> Option<(&'a KO, &'a ())> {
            Some((<$Container>::get(self, k)?.borrow(), &()))
        }

        fn is_empty(self) -> bool {
            <$Container>::is_empty(self)
        }

        fn len(self) -> usize {
            <$Container>::len(self)
        }

        fn iter(self) -> impl Iterator<Item = (&'a KO, &'a ())> {
            <$Container>::iter(self).map(|k| (k.borrow(), &()))
        }
    };
}

impl<'a, KI: Hash + Eq + Borrow<KO>, KO: Hash + Eq + ?Sized + 'a> RoMap<'a, KO, ()>
    for &'a HashSet<KI>
{
    impl_set!(HashSet<KI>);
}

impl<'a, KI: Ord + Borrow<KO>, KO: Ord + ?Sized + 'a> RoMap<'a, KO, ()> for &'a BTreeSet<KI> {
    const ITER_ORDER_SORTED: bool = true;
    impl_set!(BTreeSet<KI>);
}
