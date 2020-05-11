# cheap-ruler-rs

Port to safe Rust of [Cheap Ruler](https://github.com/mapbox/cheap-ruler) and
[Cheap Ruler C++](https://github.com/mapbox/cheap-ruler-cpp), a collection of
very fast approximations to common geodesic measurements.

**Note**: This project is currently a work in progress that lacks test coverage.
Caveat emptor!

# Usage

```rust
extern crate cheap_ruler;

use cheap_ruler::{CheapRuler, Unit};

let ruler = CheapRuler::new(44.7192003, Unit::Meters);
let dist = ruler.distance(
  &(14.8901816, 44.7209699).into(),
  &(14.8905188, 44.7209699).into()
);

assert!(dist < 38.0);
```

# TODO

* Add test coverage for all methods
* Use idiomatic Rust for methods that follow C++ conventions.

[cheap-ruler-cpp#13]: https://github.com/mapbox/cheap-ruler-cpp/pull/13
