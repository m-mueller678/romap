//! This crate provides the [`RoMap`](RoMap) trait, which describes a read-only-map data structure.
//! It is intended to allow library authors more flexibility in the types they accept:
//!
//! ```rust
//! # use romap::RoMap;
//! # struct Cat;
//! # struct Dog;
//! # trait Pet:'static{}
//! trait MyPetList {
//!     fn cats(&self) -> impl RoMap<String, Cat>;
//!     fn dogs(&self) -> impl RoMap<String, Dog>;
//!     fn all_pets(&self) -> impl RoMap<String, dyn Pet>;
//! }
//! ```
//! This crate also provides implementations for the applicable `std` collections: `{Hash|BTree}{Map|Set}`.

pub use ro_map_set::RoMapSet;

mod ro_map_set;
mod std_maps;
#[cfg(feature = "test_utils")]
pub mod test_utils;

/// A read-only-map.
pub trait RoMap<'a, K: 'a + ?Sized, V: 'a + ?Sized>: 'a + Copy {
    /// If true, then all iterators returned by methods of this trait return elements sorted by their keys.
    const ITER_ORDER_SORTED: bool = false;

    fn contains_key(self, k: &K) -> bool {
        self.get_key(k).is_some()
    }
    fn get(self, k: &K) -> Option<&'a V> {
        Some(self.get_key_value(k)?.1)
    }
    fn get_key(self, k: &K) -> Option<&'a K> {
        Some(self.get_key_value(k)?.0)
    }
    fn get_key_value(self, k: &K) -> Option<(&'a K, &'a V)>;

    fn is_empty(self) -> bool {
        self.len() == 0
    }

    fn len(self) -> usize {
        self.keys().count()
    }

    fn keys(self) -> impl 'a + Iterator<Item = &'a K> {
        self.iter().map(|(k, _)| k)
    }
    /// Values are returned in an arbitrary order, not necessarily the same order as returned by [keys](Self::keys).
    /// If [ITER_ORDER_SORTED](Self::ITER_ORDER_SORTED) is true, values are guaranteed to be ordered by their keys.
    fn values(self) -> impl 'a + Iterator<Item = &'a V> {
        self.iter().map(|(_, v)| v)
    }
    fn iter(self) -> impl 'a + Iterator<Item = (&'a K, &'a V)>;
}
