## 0.3.0 (2021-03-17)

The 0.3.0 release is a breaking change that makes the entire API type generic,
using num_traits::Float instead of fixed with f64.

### Feature changes

- The `CheapRuler` struct has been changed to `CheapRuler<T>` to use the generic
  num_traits::Float trait for floating-point numbers instead of using f64.
  Associated functions that previously operated with f64s have also had their
  argument/return types changed to T.

### Misc

- Added a distance benchmark comparing the performance of cheap ruler's distance
  to geo's haversine and vincenty implementations.

## 0.2.0 (2021-03-09)

The 0.2 release is primarily an update from geo-types 0.6 to 0.7.

### Misc

- **Breaking**: Update to geo-types 0.7, and API changes to go with it:
  - `PointOnLine<T>` requires `T` to be `std::fmt::Debug` as that trait is
    required by the underlying `geo_types::Point<T>` type.
  - `Rect<T>` is now implemented only for `geo_types::CoordNum`, not
    `geo_types::CoordinateType`. `geo_types::CoordinateType` is deprecated by
    geo-types.
- Remove dependency on float_extras. Remainder calculation for point wrapping is
  implemented with IEEE 754 formula.

## 0.1.0 (2020-12-02)

### New features

- Adds point_to_segment_distance for calculating the shortest distance from a
  point to a given line segment.
- Adds `change_unit` and `clone_with_unit` methods that allow an existing
  ruler's unit to be changed, and `distance_unit` to get the current unit used
  by the ruler.

### Feature changes

- `along` now returns an `Option<Point<f64>>` to support the case where the
  provided `LineString` is empty without a panic.
- `area` now accepts a single `Polygon` instead of a slice. The given polygon's
  interiors are subtracted from the area instead of the other elements in the
  polygon slice.
- `buffer_point`, `buffer_bbox` and `inside_bbox` APIs now take points and
  rectangles by reference instead of by value. Additionally, they accept a
  custom `Rect` type, not `geo_types::Rect`, due to that type's invariants
  causing problems when drawing a bounding box that goes over the international
  date line. A convenience `From` trait is implemented.
- `point_on_line` also now returns an `Option<PointOnLine<f64>>` to support the
  case where an empty `LineString` is provided.
- Removed `From<(Point<T>, usize, T)> for PointOnLine<T>` trait implementation,
  the `point_on_line` method now directly uses the constructor, which is saner.

### Bug fixes

- `buffer_bbox` now returns correct results for a box crossing the international
  date line.

### Misc

- Adds test coverage for all methods based on mapbox/cheap-ruler and
  mapbox/cheap-ruler-cpp implementations.
- Adds the project to github workflows with cargo clippy and fmt as additional
  test steps.
- Use `lat` and `lng` methods on `geo_types::Point` instead of `x` and `y` for
  context.


## 0.0.4 (2020-08-24)

### Misc

- Use the geo-types crate instead of geo.
- Update docs to remove the disclaimer about the WGS84-based implementation, as
  cheap-ruler-cpp and cheap-ruler now both use the same model.


## 0.0.3 (2020-05-07)

### Bug fixes

- Fix incorrect calculations for points that cross the international date line.
  See [mapbox/cheap-ruler-cpp#12] for details.

### Misc

- Internally use WGS84 ellipsoid model for calculations instead of the FCC
  formulas. See [mapbox/cheap-ruler-cpp#13] for details.


## 0.0.2 (2020-04-26)

### Feature changes

- Rename Unit struct to DistanceUnit for clarity.


<!-- References -->
[mapbox/cheap-ruler-cpp#12]: https://github.com/mapbox/cheap-ruler/pull/12
[mapbox/cheap-ruler-cpp#13]: https://github.com/mapbox/cheap-ruler/pull/13
