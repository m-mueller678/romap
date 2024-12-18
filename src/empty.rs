use crate::RoMap;

/// An empty map
#[derive(Copy, Clone)]
pub struct Empty;

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized> RoMap<'a, K, V> for Empty {
    const ITER_ORDER_SORTED: bool = true;
    fn get_key_value(self, _: &K) -> Option<(&'a K, &'a V)> {
        None
    }

    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        core::iter::empty()
    }
}
