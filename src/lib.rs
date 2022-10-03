//! # cheap-ruler
//!
//! A collection of very fast approximations to common geodesic measurements.
//! Useful for performance-sensitive code that measures things on a city scale.
//!
//! This is a port of the cheap-ruler JS library and cheap-ruler-cpp C++ library
//! into safe Rust.
//!
//! Note: WGS84 ellipsoid is used instead of the Clarke 1866 parameters used by
//! the FCC formulas. See cheap-ruler-cpp#13 for more information.

#[macro_use]
extern crate geo_types;

use geo_types::{Coordinate, LineString, Point, Polygon};
use num_traits::Float;
use std::f64;
use std::fmt;
use std::iter;
use std::mem;

mod distance_unit;
mod point_on_line;
mod rect;

pub use distance_unit::DistanceUnit;
pub use point_on_line::PointOnLine;
pub use rect::Rect;

const RE: f64 = 6378.137; // equatorial radius in km
const FE: f64 = 1.0 / 298.257223563; // flattening
const E2: f64 = FE * (2.0 - FE);

/// A collection of very fast approximations to common geodesic measurements.
/// Useful for performance-sensitive code that measures things on a city scale.
/// Point coordinates are in the [x = longitude, y = latitude] form.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq, Clone)]
pub struct CheapRuler<T>
where
    T: Float + fmt::Debug,
{
    kx: T,
    ky: T,
    dkx: T,
    dky: T,
    distance_unit: DistanceUnit,
}

impl<T> CheapRuler<T>
where
    T: Float + fmt::Debug,
{
    pub fn new(latitude: T, distance_unit: DistanceUnit) -> Self {
        let one = T::one();
        let e2 = T::from(E2).unwrap();

        // Curvature formulas from https://en.wikipedia.org/wiki/Earth_radius#Meridional
        let coslat = latitude.to_radians().cos();
        let w2 = one / (one - e2 * (one - coslat * coslat));
        let w = w2.sqrt();

        // multipliers for converting longitude and latitude degrees into distance
        let dkx = w * coslat; // based on normal radius of curvature
        let dky = w * w2 * (one - e2); // based on meridonal radius of curvature

        let (kx, ky) = calculate_multipliers(distance_unit, dkx, dky);

        Self {
            kx,
            ky,
            dkx,
            dky,
            distance_unit,
        }
    }

    /// Creates a ruler object from tile coordinates (y and z). Convenient in
    /// tile-reduce scripts
    ///
    /// # Arguments
    ///
    /// * `y` - y
    /// * `z` - z
    /// * `distance_unit` - Unit to express distances in
    ///
    /// # Examples
    ///
    /// ```
    /// use cheap_ruler::{CheapRuler, DistanceUnit};
    /// let cr = CheapRuler::<f64>::from_tile(1567, 12, DistanceUnit::Meters);
    /// ```
    pub fn from_tile(y: u32, z: u32, distance_unit: DistanceUnit) -> Self {
        assert!(z < 32);

        let n = T::from(f64::consts::PI).unwrap()
            * (T::one()
                - T::from(2.0).unwrap()
                    * (T::from(y).unwrap() + T::from(0.5).unwrap())
                    / T::from(1u32 << z).unwrap());
        let latitude = n.sinh().atan().to_degrees();

        Self::new(latitude, distance_unit)
    }

    /// Changes the ruler's unit to the given one
    ///
    /// # Arguments
    ///
    /// * `distance_unit` - New distance unit to express distances in
    pub fn change_unit(&mut self, distance_unit: DistanceUnit) {
        let (kx, ky) = calculate_multipliers(distance_unit, self.dkx, self.dky);
        self.distance_unit = distance_unit;
        self.kx = kx;
        self.ky = ky;
    }

    /// Clones the ruler to a new one with the given unit
    ///
    /// # Arguments
    ///
    /// * `distance_unit` - Distance unit to express distances in the new ruler
    pub fn clone_with_unit(&self, distance_unit: DistanceUnit) -> Self {
        let (kx, ky) = calculate_multipliers(distance_unit, self.dkx, self.dky);
        Self {
            distance_unit,
            kx,
            ky,
            dkx: self.dkx,
            dky: self.dky,
        }
    }

    /// Gets the distance unit that the ruler was instantiated with
    pub fn distance_unit(&self) -> DistanceUnit {
        self.distance_unit
    }

    /// Calculates the square of the approximate distance between two
    /// geographical points
    ///
    /// # Arguments
    ///
    /// * `a` - First point
    /// * `b` - Second point
    pub fn square_distance(&self, a: &Point<T>, b: &Point<T>) -> T {
        let dx = long_diff(a.x(), b.x()) * self.kx;
        let dy = (a.y() - b.y()) * self.ky;
        dx.powi(2) + dy.powi(2)
    }

    /// Calculates the approximate distance between two geographical points
    ///
    /// # Arguments
    ///
    /// * `a` - First point
    /// * `b` - Second point
    ///
    /// # Examples
    ///
    /// ```
    /// use cheap_ruler::{CheapRuler, DistanceUnit};
    /// let cr = CheapRuler::new(44.7192003, DistanceUnit::Meters);
    /// let dist = cr.distance(
    ///   &(14.8901816, 44.7209699).into(),
    ///   &(14.8905188, 44.7209699).into()
    /// );
    /// assert!(dist < 38.0);
    /// ```
    pub fn distance(&self, a: &Point<T>, b: &Point<T>) -> T {
        self.square_distance(a, b).sqrt()
    }

    /// Returns the bearing between two points in angles
    ///
    /// # Arguments
    ///
    /// * `a` - First point
    /// * `b` - Second point
    ///
    /// # Examples
    ///
    /// ```
    /// use cheap_ruler::{CheapRuler, DistanceUnit};
    /// let cr = CheapRuler::new(44.7192003, DistanceUnit::Meters);
    /// let bearing = cr.bearing(
    ///   &(14.8901816, 44.7209699).into(),
    ///   &(14.8905188, 44.7209699).into()
    /// );
    /// assert_eq!(bearing, 90.0);
    /// ```
    pub fn bearing(&self, a: &Point<T>, b: &Point<T>) -> T {
        let dx = long_diff(b.x(), a.x()) * self.kx;
        let dy = (b.y() - a.y()) * self.ky;

        dx.atan2(dy).to_degrees()
    }

    /// Returns a new point given distance and bearing from the starting point
    ///
    /// # Arguments
    ///
    /// * `origin` - origin point
    /// * `dist` - distance
    /// * `bearing` - bearing
    ///
    /// # Examples
    ///
    /// ```
    /// use cheap_ruler::{CheapRuler, DistanceUnit};
    /// let cr = CheapRuler::new(44.7192003, DistanceUnit::Meters);
    /// let p1 = (14.8901816, 44.7209699).into();
    /// let p2 = (14.8905188, 44.7209699).into();
    /// let dist = cr.distance(&p1, &p2);
    /// let bearing = cr.bearing(&p1, &p2);
    /// let destination = cr.destination(&p1, dist, bearing);
    ///
    /// assert_eq!(destination.x(), p2.x());
    /// assert_eq!(destination.y(), p2.y());
    /// ```
    pub fn destination(
        &self,
        origin: &Point<T>,
        dist: T,
        bearing: T,
    ) -> Point<T> {
        let a = bearing.to_radians();
        self.offset(origin, a.sin() * dist, a.cos() * dist)
    }

    /// Returns a new point given easting and northing offsets (in ruler units)
    /// from the starting point
    ///
    /// # Arguments
    ///
    /// * `origin` - point
    /// * `dx` - easting
    /// * `dy` - northing
    pub fn offset(&self, origin: &Point<T>, dx: T, dy: T) -> Point<T> {
        (origin.x() + dx / self.kx, origin.y() + dy / self.ky).into()
    }

    /// Given a line (an array of points), returns the total line distance.
    ///
    /// # Arguments
    ///
    /// * `points` - line of points
    ///
    /// # Example
    ///
    /// ```
    /// use cheap_ruler::{CheapRuler, DistanceUnit};
    /// use geo_types::LineString;
    /// let cr = CheapRuler::new(50.458, DistanceUnit::Meters);
    /// let line_string: LineString<f64> = vec![
    ///     (-67.031, 50.458),
    ///     (-67.031, 50.534),
    ///     (-66.929, 50.534),
    ///     (-66.929, 50.458)
    /// ].into();
    /// let length = cr.line_distance(&line_string);
    /// ```
    pub fn line_distance(&self, points: &LineString<T>) -> T {
        let line_iter = points.0.iter().copied();

        let left = iter::once(None).chain(line_iter.clone().map(Some));
        left.zip(line_iter)
            .map(|(a, b)| match a {
                Some(a) => self.distance(&a.into(), &b.into()),
                None => T::zero(),
            })
            // avoided using Iterator's sum() so that we don't have to require T
            // to implement std::iter::Sum.
            .fold(T::zero(), |acc, x| acc + x)
    }

    /// Given a polygon returns the area
    ///
    /// * `polygon` - Polygon
    pub fn area(&self, polygon: &Polygon<T>) -> T {
        let exterior_sum =
            sum_area(&polygon.exterior().points().collect::<Vec<Point<T>>>());
        let interiors_sum = polygon
            .interiors()
            .iter()
            .map(|interior| {
                sum_area(&interior.points().collect::<Vec<Point<T>>>())
            })
            .fold(T::zero(), |acc, x| acc + x);
        let sum = exterior_sum - interiors_sum;
        (sum.abs() / T::from(2.0).unwrap()) * self.kx * self.ky
    }

    /// Returns the point at a specified distance along the line
    ///
    /// # Arguments
    ///
    /// * `line` - Line
    /// * `dist` - Distance along the line
    pub fn along(&self, line: &LineString<T>, dist: T) -> Option<Point<T>> {
        let line_len = line.0.len();
        if line_len == 0 {
            return None;
        }

        if dist <= T::zero() {
            return Some(line[0].into());
        }

        let last_index = line_len - 1;
        let mut sum = T::zero();
        for i in 0..last_index {
            let p0 = &line[i].into();
            let p1 = &line[i + 1].into();
            let d = self.distance(p0, p1);
            sum = sum + d;
            if sum > dist {
                return Some(interpolate(p0, p1, (dist - (sum - d)) / d));
            }
        }
        Some(line[last_index].into())
    }

    /// Returns the shortest distance between a point and a line segment given
    /// with two points.
    ///
    /// # Arguments
    ///
    /// * `p` - Point to calculate the distance from
    /// * `start` - Start point of line segment
    /// * `end` - End point of line segment
    pub fn point_to_segment_distance(
        &self,
        p: &Point<T>,
        start: &Point<T>,
        end: &Point<T>,
    ) -> T {
        let zero = T::zero();
        let mut x = start.x();
        let mut y = start.y();
        let dx = long_diff(end.x(), x) * self.kx;
        let dy = (end.y() - y) * self.ky;

        if dx != zero || dy != zero {
            let t = (long_diff(p.x(), x) * self.kx * dx
                + (p.y() - y) * self.ky * dy)
                / (dx * dx + dy * dy);
            if t > T::one() {
                x = end.x();
                y = end.y();
            } else if t > zero {
                x = x + (dx / self.kx) * t;
                y = y + (dy / self.ky) * t;
            }
        }
        self.distance(p, &point!(x: x, y: y))
    }

    /// Returns a tuple of the form (point, index, t) where point is closest
    /// point on the line from the given point, index is the start index of the
    /// segment with the closest point, and t is a parameter from 0 to 1 that
    /// indicates where the closest point is on that segment
    ///
    /// # Arguments
    ///
    /// * `line` - Line to compare with point
    /// * `point` - Point to calculate the closest point on the line
    pub fn point_on_line(
        &self,
        line: &LineString<T>,
        point: &Point<T>,
    ) -> Option<PointOnLine<T>> {
        let zero = T::zero();
        let mut min_dist = T::infinity();
        let mut min_x = zero;
        let mut min_y = zero;
        let mut min_i = 0;
        let mut min_t = zero;

        let line_len = line.0.len();
        if line_len == 0 {
            return None;
        }

        for i in 0..line_len - 1 {
            let mut t = zero;
            let mut x = line[i].x;
            let mut y = line[i].y;
            let dx = long_diff(line[i + 1].x, x) * self.kx;
            let dy = (line[i + 1].y - y) * self.ky;

            if dx != zero || dy != zero {
                t = (long_diff(point.x(), x) * self.kx * dx
                    + (point.y() - y) * self.ky * dy)
                    / (dx * dx + dy * dy);

                if t > T::one() {
                    x = line[i + 1].x;
                    y = line[i + 1].y;
                } else if t > zero {
                    x = x + (dx / self.kx) * t;
                    y = y + (dy / self.ky) * t;
                }
            }

            let d2 = self.square_distance(point, &point!(x: x, y: y));

            if d2 < min_dist {
                min_dist = d2;
                min_x = x;
                min_y = y;
                min_i = i;
                min_t = t;
            }
        }

        Some(PointOnLine::new(
            point!(x: min_x, y: min_y),
            min_i,
            T::zero().max(T::one().min(min_t)),
        ))
    }

    /// Returns a part of the given line between the start and the stop points
    /// (or their closest points on the line)
    ///
    /// # Arguments
    ///
    /// * `start` - Start point
    /// * `stop` - Stop point
    /// * `line` - Line string
    pub fn line_slice(
        &self,
        start: &Point<T>,
        stop: &Point<T>,
        line: &LineString<T>,
    ) -> LineString<T> {
        let pol1 = self.point_on_line(line, start);
        let pol2 = self.point_on_line(line, stop);

        if pol1.is_none() || pol2.is_none() {
            return line_string![];
        }
        let mut pol1 = pol1.unwrap();
        let mut pol2 = pol2.unwrap();

        if pol1.index() > pol2.index()
            || pol1.index() == pol2.index() && pol1.t() > pol2.t()
        {
            mem::swap(&mut pol1, &mut pol2);
        }

        let mut slice = vec![pol1.point()];

        let l = pol1.index() + 1;
        let r = pol2.index();

        if line[l] != slice[0].into() && l <= r {
            slice.push(line[l].into());
        }

        let mut i = l + 1;
        while i <= r {
            slice.push(line[i].into());
            i += 1;
        }

        if line[r] != pol2.point().into() {
            slice.push(pol2.point());
        }

        slice.into()
    }

    /// Returns a part of the given line between the start and the stop points
    /// indicated by distance along the line
    ///
    /// * `start` - Start distance
    /// * `stop` - Stop distance
    /// * `line` - Line string
    pub fn line_slice_along(
        &self,
        start: T,
        stop: T,
        line: &LineString<T>,
    ) -> LineString<T> {
        let mut sum = T::zero();
        let mut slice = vec![];

        let line_len = line.0.len();
        if line_len == 0 {
            return slice.into();
        }

        for i in 0..line_len - 1 {
            let p0 = line[i].into();
            let p1 = line[i + 1].into();
            let d = self.distance(&p0, &p1);

            sum = sum + d;

            if sum > start && slice.is_empty() {
                slice.push(interpolate(&p0, &p1, (start - (sum - d)) / d));
            }

            if sum >= stop {
                slice.push(interpolate(&p0, &p1, (stop - (sum - d)) / d));
                return slice.into();
            }

            if sum > start {
                slice.push(p1);
            }
        }

        slice.into()
    }

    /// Given a point, returns a bounding rectangle created from the given point
    /// buffered by a given distance
    ///
    /// # Arguments
    ///
    /// * `p` - Point
    /// * `buffer` - Buffer distance
    pub fn buffer_point(&self, p: &Point<T>, buffer: T) -> Rect<T> {
        let v = buffer / self.ky;
        let h = buffer / self.kx;

        Rect::new(
            Coordinate {
                x: p.x() - h,
                y: p.y() - v,
            },
            Coordinate {
                x: p.x() + h,
                y: p.y() + v,
            },
        )
    }

    /// Given a bounding box, returns the box buffered by a given distance
    ///
    /// # Arguments
    ///
    /// * `bbox` - Bounding box
    /// * `buffer` - Buffer distance
    pub fn buffer_bbox(&self, bbox: &Rect<T>, buffer: T) -> Rect<T> {
        let v = buffer / self.ky;
        let h = buffer / self.kx;

        Rect::new(
            Coordinate {
                x: bbox.min().x - h,
                y: bbox.min().y - v,
            },
            Coordinate {
                x: bbox.max().x + h,
                y: bbox.max().y + v,
            },
        )
    }

    /// Returns true if the given point is inside in the given bounding box,
    /// otherwise false.
    ///
    /// # Arguments
    ///
    /// * `p` - Point
    /// * `bbox` - Bounding box
    pub fn inside_bbox(&self, p: &Point<T>, bbox: &Rect<T>) -> bool {
        p.y() >= bbox.min().y
            && p.y() <= bbox.max().y
            && long_diff(p.x(), bbox.min().x) >= T::zero()
            && long_diff(p.x(), bbox.max().x) <= T::zero()
    }
}

pub fn interpolate<T: Float + fmt::Debug>(
    a: &Point<T>,
    b: &Point<T>,
    t: T,
) -> Point<T> {
    let dx = long_diff(b.x(), a.x());
    let dy = b.y() - a.y();
    Point::new(a.x() + dx * t, a.y() + dy * t)
}

fn calculate_multipliers<T: Float>(
    distance_unit: DistanceUnit,
    dkx: T,
    dky: T,
) -> (T, T) {
    let re = T::from(RE).unwrap();
    let mul = distance_unit
        .conversion_factor_kilometers::<T>()
        .to_radians()
        * re;
    let kx = mul * dkx;
    let ky = mul * dky;
    (kx, ky)
}

fn long_diff<T: Float>(a: T, b: T) -> T {
    let threesixty = T::from(360).unwrap();
    let diff = a - b;
    diff - ((diff / threesixty).round() * threesixty)
}

fn sum_area<T: Float + fmt::Debug>(line: &[Point<T>]) -> T {
    let line_len = line.len();
    let mut sum = T::zero();
    let mut k = line_len - 1;
    for j in 0..line_len {
        sum = sum + (line[j].x() - line[k].x()) * (line[j].y() + line[k].y());
        k = j;
    }
    sum
}
