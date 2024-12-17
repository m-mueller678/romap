use romap::RoMapSet;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn test_iter(size: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..size).map(move |x| (x, size * 2 + x))
}

fn test_iter_set(size: usize) -> impl Iterator<Item = usize> {
    test_iter(size).map(|x| x.0)
}

#[test]
fn test() {
    for size in [0, 1, 10, 1000] {
        let btree_map = test_iter(size).collect::<BTreeMap<_, _>>();
        romap::test_utils::test_map(&btree_map, &size);
        let btree_set = test_iter_set(size).collect::<BTreeSet<_>>();
        romap::test_utils::test_map(&btree_set, &size);
        romap::test_utils::test_iter_eq(RoMapSet::from(&btree_map), &btree_set, false);

        let hash_map = test_iter(size).collect::<HashMap<_, _>>();
        romap::test_utils::test_map(&hash_map, &size);
        let hash_set = test_iter_set(size).collect::<HashSet<_>>();
        romap::test_utils::test_map(&hash_set, &size);
        romap::test_utils::test_iter_eq(RoMapSet::from(&hash_map), &hash_set, false);

        romap::test_utils::test_iter_eq(&hash_map, &btree_map, false);
        romap::test_utils::test_iter_eq(RoMapSet::from(&hash_set), &hash_set, true);
    }
}
