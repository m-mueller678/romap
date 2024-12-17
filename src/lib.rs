pub use ro_map_set::RoMapSet;

mod ro_map_set;
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
    fn get_key(self, j: &K) -> Option<&'a K> {
        Some(self.get_key_value(j)?.0)
    }
    fn get_key_value(self, j: &K) -> Option<(&'a K, &'a V)>;

    fn is_empty(self) -> bool {
        self.len() == 0
    }

    fn len(self) -> usize {
        self.keys().count()
    }

    fn keys(self) -> impl Iterator<Item = &'a K> {
        self.iter().map(|(k, _)| k)
    }
    fn values(self) -> impl Iterator<Item = &'a V> {
        self.iter().map(|(_, v)| v)
    }
    fn iter(self) -> impl Iterator<Item = (&'a K, &'a V)>;
}