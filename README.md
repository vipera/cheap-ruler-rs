[![GitHub Actions][actions badge]][actions]
![GitHub License][license badge]

# cheap-ruler-rs

Port to safe Rust of [Cheap Ruler](https://github.com/mapbox/cheap-ruler) and
[Cheap Ruler C++](https://github.com/mapbox/cheap-ruler-cpp), a collection of
very fast approximations to common geodesic measurements.

The approximations are based on WGS84 and its ellipsoidal model of the Earth.
The results of calculations are accurate to small differences of the latitude
provided at construction, and are less computationally expensive. See Cheap
Ruler for accuracy claims compared to Vincenty formulas.

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

# Changelog

See the [CHANGELOG](CHANGELOG.md) file for details.

# License

This library is licensed under the ISC License. See the [LICENSE](LICENSE) file
for the full license content.

# TODO

* Use idiomatic Rust for methods that follow C++ conventions.
* Benchmarks

<!-- Badges -->
[actions badge]: https://img.shields.io/github/workflow/status/vipera/cheap-ruler-rs/CI?style=flat-square
[actions]: https://github.com/vipera/cheap-ruler-rs/actions?query=workflow%3ACI
[license badge]: https://img.shields.io/github/license/vipera/cheap-ruler-rs?style=flat-square
