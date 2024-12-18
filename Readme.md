# romap

The [`RoMap`](RoMap) trait describes a read-only-map.
It is intended to allow library authors more flexibility in the types they accept.

This crate includes combinators and implementations for common containers.

```rust
use romap::{deref_value, project_value, union, RoMap};

trait MyPetListTrait {
    fn cats(&self) -> impl RoMap<str, Cat>;
    fn dogs(&self) -> impl RoMap<str, Dog>;
    fn all_pets(&self) -> impl RoMap<str, dyn Pet> {
        union(
            project_value(self.cats(), |p| p as &dyn Pet),
            project_value(self.dogs(), |p| p as &dyn Pet),
        )
    }
}

struct MyPetListImpl {
    cats: HashMap<String, Cat>,
    dogs: BTreeMap<&'static str, Box<Dog>>,
}

impl MyPetListTrait for MyPetListImpl {
    fn cats(&self) -> impl RoMap<str, Cat> {
        &self.cats
    }

    fn dogs(&self) -> impl RoMap<str, Dog> {
        deref_value(&self.dogs)
    }
}
```

License: MIT OR Apache-2.0
