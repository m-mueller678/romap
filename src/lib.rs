#![cfg_attr(not(feature = "std"), no_std)]
#![allow(rustdoc::redundant_explicit_links)]
//! The [`RoMap`](RoMap) trait describes a read-only-map.
//! It is intended to allow library authors more flexibility in the types they accept.
//!
//! This crate includes combinators and implementations for common containers.
//!
//! ```rust
//! # use std::collections::{BTreeMap, HashMap};
//! use romap::{deref_value, project_value, union, RoMap};
//! # struct Cat;
//! # struct Dog;
//! # trait Pet:'static{}
//! # impl Pet for Dog{}
//! # impl Pet for Cat{}
//!
//! trait MyPetListTrait {
//!     fn cats(&self) -> impl RoMap<str, Cat>;
//!     fn dogs(&self) -> impl RoMap<str, Dog>;
//!     fn all_pets(&self) -> impl RoMap<str, dyn Pet> {
//!         union(
//!             project_value(self.cats(), |p| p as &dyn Pet),
//!             project_value(self.dogs(), |p| p as &dyn Pet),
//!         )
//!     }
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
//! }
//! ```

pub use discard_values::discard_values;
pub use empty::Empty;
pub use union::union;
pub use value_projection::{deref_value, project_value};
mod discard_values;

/// The concrete combinator types
pub mod structs {
    pub use crate::discard_values::DiscardValues;
    pub use crate::union::Union;
}
mod either;
mod empty;
mod option;
#[cfg(feature = "std")]
mod std_maps;
#[cfg(feature = "test_utils")]
pub mod test_utils;
mod union;
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
