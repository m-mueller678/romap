# romap

This crate provides the [`RoMap`](RoMap) trait, which describes a read-only-map data structure.
It is intended to allow library authors more flexibility in the types they accept:

```rust
trait MyPetList {
    fn cats(&self) -> impl RoMap<str, Cat>;
    fn dogs(&self) -> impl RoMap<str, Dog>;
    fn all_pets(&self) -> impl RoMap<str, dyn Pet>;
}
```
This crate also provides implementations for the applicable `std` collections: `{Hash|BTree}{Map|Set}`.

License: MIT OR Apache-2.0
