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

## geo_types

The library uses the geo-types crate for representation of points and
coordinates. The one exception are the BBox functions, which use a custom `Rect`
implementation that does not automatically swap min/max bounds:

```rust
extern crate cheap_ruler;
#[macro_use] extern crate geo_types;

use cheap_ruler::{CheapRuler, DistanceUnit, Rect};
use geo_types::Coordinate;

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
crate's haversine algorithm implementation. Bearing calculation is on par with
geo's implementation, and destination is over 3 times faster.

Times below are from a machine with Intel Core i7-8550U.

```
distance/cheap_ruler    time:   [291.10 ps 292.38 ps 293.89 ps]
distance/haversine      time:   [4.7215 ns 4.8545 ns 5.0086 ns]
distance/vincenty       time:   [315.83 ns 320.75 ns 325.93 ns]

bearing/cheap_ruler     time:   [16.201 ns 16.239 ns 16.281 ns]
bearing/geo             time:   [16.523 ns 16.576 ns 16.629 ns]

destination/cheap_ruler time:   [24.984 ns 25.183 ns 25.471 ns]
destination/haversine   time:   [82.251 ns 82.670 ns 83.169 ns]
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
[actions badge]: https://img.shields.io/github/actions/workflow/status/vipera/cheap-ruler-rs/rust.yml?branch=master&style=flat-square
[actions]: https://github.com/vipera/cheap-ruler-rs/actions?query=workflow%3ACI
[license badge]: https://img.shields.io/github/license/vipera/cheap-ruler-rs?style=flat-square
