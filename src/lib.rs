#![cfg_attr(not(feature = "std"), no_std)]

//! This crate provides the [`RoMap`](RoMap) trait, which describes a read-only-map data structure.
//! It is intended to allow library authors more flexibility in the types they accept:
//!
//! ```rust
//! # use std::collections::{BTreeMap, HashMap};
//! use romap::{deref_value, RoMap};
//! # struct Cat;
//! # struct Dog;
//! # trait Pet:'static{}
//! trait MyPetListTrait {
//!     fn cats(&self) -> impl RoMap<str, Cat>;
//!     fn dogs(&self) -> impl RoMap<str, Dog>;
//!     fn all_pets(&self) -> impl RoMap<str, dyn Pet>;
//! }
//!
//! struct MyPetListImpl {
//!     cats: HashMap<String, Cat>,
//!     dogs: BTreeMap<&'static str, Box<Dog>>,
//! }
//!
//! impl MyPetListTrait for MyPetListImpl {
//!     fn cats(&self) -> impl RoMap<str, Cat> {
//!         &self.cats
//!     }
//!
//!     fn dogs(&self) -> impl RoMap<str, Dog> {
//!         deref_value(&self.dogs)
//!     }
//!
//!     fn all_pets(&self) -> impl RoMap<str, dyn Pet> {
//!         todo!()
//!     }
//! }
//! ```
//! This crate also provides implementations for the applicable `std` collections: `{Hash|BTree}{Map|Set}`.

pub use ro_map_set::RoMapSet;
pub use value_projection::{deref_value, project_value};
mod ro_map_set;
#[cfg(feature = "std")]
mod std_maps;
#[cfg(feature = "test_utils")]
pub mod test_utils;
mod value_projection;

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
