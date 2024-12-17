use crate::RoMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn test_map<
    'a,
    K: Hash + Eq + Copy + Debug + Ord + 'a,
    V: Eq + Debug + 'a,
    M: RoMap<'a, K, V>,
>(
    m: M,
    not_contained: &K,
) {
    assert_eq!(m.is_empty(), m.len() == 0);
    let assert_contained = |k, should: Option<&V>| {
        assert_eq!(m.contains_key(k), should.is_some());
        assert_eq!(m.get(k), should);
        assert_eq!(m.get_key(k).is_some(), should.is_some());
        assert_eq!(m.get_key_value(k).map(|x| x.1), should);
    };

    let mut max_seen = None;
    let mut seen = HashSet::new();
    let mut keys = m.keys();
    let mut vals = m.values();
    for (k, v) in m.iter() {
        if M::ITER_ORDER_SORTED {
            assert!(Some(k) > max_seen);
            max_seen = Some(k);
        }
        assert_contained(k, Some(v));
        assert!(seen.insert(*k));
        assert_eq!(keys.next(), Some(k));
        assert_eq!(vals.next(), Some(v));
    }
    assert_eq!(keys.next(), None);
    assert_eq!(vals.next(), None);
    assert_contained(not_contained, None);
}

pub fn test_iter_eq<
    'a,
    K: Eq + Ord + Debug + 'a,
    V: Eq + Debug + 'a,
    M1: RoMap<'a, K, V>,
    M2: RoMap<'a, K, V>,
>(
    m1: M1,
    m2: M2,
    mut enforce_order: bool,
) {
    enforce_order |= M1::ITER_ORDER_SORTED && M2::ITER_ORDER_SORTED;
    let mut v1 = m1.iter().collect::<Vec<_>>();
    let mut v2 = m2.iter().collect::<Vec<_>>();
    if !enforce_order {
        v1.sort_unstable_by_key(|x| x.0);
        v2.sort_unstable_by_key(|x| x.0);
    }
    assert_eq!(v1, v2);
}
