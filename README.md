# cheap-ruler-rs

Port to safe Rust of [Cheap Ruler](https://github.com/mapbox/cheap-ruler) and
[Cheap Ruler C++](https://github.com/mapbox/cheap-ruler-cpp), a collection of
very fast approximations to common geodesic measurements.

**Note**: This project is currently a work in progress that lacks test coverage.
Caveat emptor!

This project uses slightly different forumulas to the Mapbox projects. The WGS84
ellipsoid is used instead of the Clarke 1866 parameters (as in the FCC
formulas). See [cheap-ruler-cpp#13] for more information on this choice.

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
