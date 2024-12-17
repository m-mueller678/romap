use core::hash::Hash;

mod ro_map_set;
pub use ro_map_set::RoMapSet;

#[cfg(feature = "std")]
mod std_maps;

pub trait RoMap<'a, K: 'a, V: 'a>: Copy {
    const ITER_ORDER_DETERMINISTIC: bool = false;
    const ITER_ORDER_SORTED: bool = false;

    fn contains_key(self, j: &K) -> bool {
        self.get_key(j).is_some()
    }
    fn get(self, j: &K) -> Option<&'a V> {
        Some(self.get_key_value(j)?.1)
    }

    fn len(self) -> usize {
        self.keys().count()
    }

    fn get_key(self, j: &K) -> Option<&'a K> {
        Some(self.get_key_value(j)?.0)
    }
    fn get_key_value(self, j: &K) -> Option<(&'a K, &'a V)>;
    fn keys(self) -> impl Iterator<Item = &'a K> {
        self.iter().map(|(k, v)| k)
    }
    fn values(self) -> impl Iterator<Item = &'a V> {
        self.iter().map(|(k, v)| v)
    }
    fn iter(self) -> impl Iterator<Item = (&'a K, &'a V)>;
}
