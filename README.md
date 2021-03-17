[![GitHub Actions][actions badge]][actions]
[![GitHub License][license badge]][LICENSE]

# cheap-ruler-rs

Port to safe Rust of [cheap-ruler] and [cheap-ruler-cpp], a collection of very
fast approximations to common geodesic measurements.

The approximations are based on WGS84 and its ellipsoidal model of the Earth.
The results of calculations are accurate to small differences of the latitude
provided at construction, and are less computationally expensive.

See [cheap-ruler]'s readme for accuracy claims compared to the Vincenty
formulas.

# Usage

Here's example program to print the distance and bearing between two points:

```rust
extern crate cheap_ruler;
#[macro_use] extern crate geo_types;

use cheap_ruler::{CheapRuler, DistanceUnit};

fn main() {
  let ruler = CheapRuler::new(44.7192003, DistanceUnit::Meters);
  let p1 = point!(x: 14.8901816, y: 44.7209699);
  let p2 = point!(x: 14.8905188, y: 44.7209699);

  let dist = ruler.distance(&p1, &p2);
  let bearing = ruler.bearing(&p1, &p2);

  println!("Distance between points: {}", dist);
  println!("Bearing: {}", bearing);
}
```

## Unit of distance

This Rust port additionally allows the distance unit of the ruler to be changed
and retrieved at any point after construction at the cost of larger memory size
of the ruler struct itself (40 bytes instead of 16 bytes), but with no
additional overhead.

```rust
extern crate cheap_ruler;
use cheap_ruler::{CheapRuler, DistanceUnit};

fn main() {
  let mut ruler = CheapRuler::new(44.7192003, DistanceUnit::Meters);
  println!("Distance unit: {:?}", ruler.distance_unit());

  ruler.change_unit(DistanceUnit::Miles);
  println!("Distance unit: {:?}", ruler.distance_unit());
}
```

## geo_types

The library uses the geo-types crate for representation of points and
coordinates. The one exception are the BBox functions, which use a custom `Rect`
implementation that does not automatically swap min/max bounds:

```rust
extern crate cheap_ruler;
#[macro_use] extern crate geo_types;

use cheap_ruler::{CheapRuler, DistanceUnit, Rect};
use geo_types::{Coordinate};

fn main() {
  let ruler = CheapRuler::new(32.8351, DistanceUnit::Kilometers);
  let bbox = Rect::new(
      Coordinate { x: 179.9, y: 32.7 },
      Coordinate { x: -179.9, y: 32.9 },
  );
  let p = point!(x: 180.0, y: 32.8);
  assert!(ruler.inside_bbox(&p, &bbox));
}
```

# Benchmarks

Cheap ruler's distance calculation is about 16 times faster than the [geo]
crate's haversine algorithm implementation (times below are from a machine with
Intel Core i7-8550U).

```
distance/cheap_ruler    time:   [291.10 ps 292.38 ps 293.89 ps]
distance/haversine      time:   [4.7215 ns 4.8545 ns 5.0086 ns]
distance/vincenty       time:   [315.83 ns 320.75 ns 325.93 ns]
```

# Changelog

See the [CHANGELOG] file for details.

# License

This library is licensed under the ISC License. See the [LICENSE] file for the
full license content.

# TODO

* Use idiomatic Rust for methods that follow C++ conventions.

<!-- References -->
[cheap-ruler]: https://github.com/mapbox/cheap-ruler
[cheap-ruler-cpp]: https://github.com/mapbox/cheap-ruler-cpp
[geo]: https://github.com/georust/geo
[LICENSE]: LICENSE
[CHANGELOG]: CHANGELOG.md

<!-- Badges -->
[actions badge]: https://img.shields.io/github/workflow/status/vipera/cheap-ruler-rs/CI?style=flat-square
[actions]: https://github.com/vipera/cheap-ruler-rs/actions?query=workflow%3ACI
[license badge]: https://img.shields.io/github/license/vipera/cheap-ruler-rs?style=flat-square
