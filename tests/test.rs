use romap::{RoMap, RoMapSet};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;

fn test_iter(size: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..size).map(move |x| (x, size * 2 + x))
}

#[test]
fn test_std() {
    for size in [0, 1, 10, 1000] {
        let btree_map = test_iter(size).collect::<BTreeMap<_, _>>();
        romap::test_utils::test_map(&btree_map, &size);
        let hash_map = test_iter(size).collect::<HashMap<_, _>>();
        romap::test_utils::test_map(&hash_map, &size);
    }
}

#[test]
fn test_map_set() {
    for size in [0, 1, 10, 1000] {
        let btree_map = test_iter(size).collect::<BTreeMap<_, _>>();
        let btree_set = test_iter(size)
            .map(|x| (x.0, ()))
            .collect::<BTreeMap<_, _>>();
        romap::test_utils::test_map(RoMapSet::from(&btree_map), &size);
        romap::test_utils::test_iter_eq(RoMapSet::from(&btree_map), &btree_set, true);
        romap::test_utils::test_iter_eq(RoMapSet::from(&btree_set), &btree_set, true);
    }
}
