extern crate cheap_ruler;
#[macro_use]
extern crate geo_types;
#[macro_use]
extern crate lazy_static;

#[cfg_attr(test, macro_use)]
mod common;
mod fixtures;

use cheap_ruler::{CheapRuler, DistanceUnit, Rect};
pub use common::*;
use fixtures::lines::{LINES, POINTS};
use fixtures::turf;
use geo_types::{Coordinate, LineString, Polygon};

#[test]
fn test_change_unit() {
    let mut ruler = fixtures::ruler_km();

    let p1 = point!(x: 30.5, y: 32.8351);
    let p2 = point!(x: 30.51, y: 32.8451);

    let d = ruler.distance(&p1, &p2);
    ruler.change_unit(DistanceUnit::Meters);
    let d2 = ruler.distance(&p1, &p2);

    assert_eq_err!(1000.0, d2 / d, 1e-12);
}

#[test]
fn test_clone_with_unit() {
    let ruler = fixtures::ruler_km();

    let p1 = point!(x: 30.5, y: 32.8351);
    let p2 = point!(x: 30.51, y: 32.8451);

    // the operation should be the same as cloning the ruler, then changing its
    // unit
    let mut r1 = ruler.clone();
    r1.change_unit(DistanceUnit::Meters);
    let r2 = ruler.clone_with_unit(DistanceUnit::Meters);

    let d = ruler.distance(&p1, &p2);
    let d1 = r1.distance(&p1, &p2);
    let d2 = r2.distance(&p1, &p2);

    assert_eq_err!(1.0, d2 / d1, 1e-12);
    assert_eq_err!(1000.0, d1 / d, 1e-12);
    assert_eq_err!(1000.0, d2 / d, 1e-12);
}

#[test]
fn test_distance() {
    let ruler = fixtures::ruler_km();

    for i in 0..POINTS.len() - 1 {
        let expected = turf::DISTANCE[i];
        let actual = ruler.distance(&POINTS[i], &POINTS[i + 1]);

        assert_eq_err!(expected, actual, 0.003)
    }
}

#[test]
fn test_distance_over_dateline() {
    let ruler = fixtures::ruler_km();

    let p0 = point!(x: 179.9, y: 32.7);
    let p1 = point!(x: -179.9, y: 32.9);

    let actual = ruler.distance(&p0, &p1);

    assert_eq_err!(29.05, actual, 0.001, "Distance over dateline within 0.1%");
}

#[test]
fn test_distance_miles() {
    let ruler = fixtures::ruler_km();
    let ruler_miles = fixtures::ruler_miles();

    let p1 = point!(x: 30.5, y: 32.8351);
    let p2 = point!(x: 30.51, y: 32.8451);

    let d = ruler.distance(&p1, &p2);
    let d2 = ruler_miles.distance(&p1, &p2);

    assert_eq_err!(1.609344, d / d2, 1e-12);
}

#[test]
fn test_bearing() {
    let ruler = fixtures::ruler_km();

    for i in 0..POINTS.len() - 1 {
        let expected = turf::BEARING[i];
        let actual = ruler.bearing(&POINTS[i], &POINTS[i + 1]);

        assert_eq_err!(expected, actual, 0.005);
    }
}

#[test]
fn test_destination() {
    let ruler = fixtures::ruler_km();

    for i in 0..POINTS.len() {
        let bearing = (i % 360) as f64 - 180.0;
        let expected = turf::DESTINATION[i];

        let actual = ruler.destination(&POINTS[i], 1.0, bearing);

        assert_eq_err!(expected.x(), actual.x(), 1e-6); // longitude
        assert_eq_err!(expected.y(), actual.y(), 1e-6); // latitude
    }
}

#[test]
fn test_line_distance_empty() {
    let ruler = fixtures::ruler_km();

    let empty_line = line_string![];
    let expected = 0.0;
    let actual = ruler.line_distance(&empty_line);

    assert_eq_err!(expected, actual, 0.0);
}

#[test]
fn test_line_distance() {
    let ruler = fixtures::ruler_km();

    for i in 0..LINES.len() {
        let expected = turf::LINEDISTANCE[i];
        let actual = ruler.line_distance(&LINES[i]);

        assert_eq_err!(expected, actual, 0.003);
    }
}

#[test]
fn test_area() {
    let ruler = fixtures::ruler_km();

    let mut j = 0;
    for i in 0..LINES.len() {
        if LINES[i].points().count() < 3 {
            continue;
        }

        let mut ring = LINES[i].to_owned().into_points();
        ring.push(point!(x: LINES[i][0].x, y: LINES[i][0].y));

        let polygon = Polygon::new(LineString::from(ring), vec![]);
        let expected = turf::AREA[j];
        let actual = ruler.area(&polygon);

        assert_eq_err!(expected, actual, 0.003);

        j += 1;
    }
}

#[test]
fn test_area_subtractions() {
    let ruler = fixtures::ruler_km();
    let zone_outer = line_string![
        (x: 71.329356, y: -6.19387),
        (x: 71.353358, y: -6.172451),
        (x: 71.368091, y: -6.172537),
        (x: 71.401737, y: -6.189896),
        (x: 71.301494, y: -6.31167),
        (x: 71.276782, y: -6.293861),
        (x: 71.295337, y: -6.242543),
        (x: 71.305667, y: -6.223857),
    ];
    let zone_inner = line_string![
        (x: 71.331072, y: -6.217463),
        (x: 71.358165, y: -6.204493),
        (x: 71.351612, y: -6.227702),
        (x: 71.334102, y: -6.249496),
        (x: 71.316666, y: -6.26014),
        (x: 71.319829, y: -6.235973),
    ];

    // measurea area of polygon
    let polygon = Polygon::new(zone_outer.clone(), vec![]);
    let expected = 75.3123;
    let actual = ruler.area(&polygon);
    assert_eq_err!(expected, actual, 0.003);

    // measure area of exclusion polygon
    let polygon_exclusion = Polygon::new(zone_inner.clone(), vec![]);
    let expected_exclusion = 10.2519;
    let actual_exclusion = ruler.area(&polygon_exclusion);
    assert_eq_err!(expected_exclusion, actual_exclusion, 0.003);

    // measure area of polygon with excluded part
    let polygon_subtracted = Polygon::new(zone_outer, vec![zone_inner]);
    let expected_subtracted = expected - expected_exclusion;
    let actual_subtracted = ruler.area(&polygon_subtracted);
    assert_eq_err!(expected_subtracted, actual_subtracted, 0.003);
}

#[test]
fn test_along() {
    let ruler = fixtures::ruler_km();

    for i in 0..LINES.len() {
        let expected = turf::ALONG[i];
        let actual = ruler
            .along(&LINES[i], turf::ALONG_DIST[i])
            .expect("Non-empty line string given");

        assert_eq_err!(expected.x(), actual.x(), 1e-6); // along longitude
        assert_eq_err!(expected.y(), actual.y(), 1e-6); // along latitude
    }
}

#[test]
fn test_along_empty() {
    let ruler = fixtures::ruler_km();

    let empty_line = line_string![];
    let actual = ruler.along(&empty_line, 0.0);

    assert!(actual.is_none());
}

#[test]
fn test_along_with_dist_lt_0() {
    let ruler = fixtures::ruler_km();

    let coord = LINES[0][0];
    let actual = ruler
        .along(&LINES[0], -5.0)
        .expect("Non-empty line string given");

    assert_eq!(point!(x: coord.x, y: coord.y), actual);
}

#[test]
fn test_along_with_dist_greater_than_length() {
    let ruler = fixtures::ruler_km();

    let coord = LINES[0].points().last().expect("Last element");
    let actual = ruler
        .along(&LINES[0], 1000.0)
        .expect("Non-empty line string given");

    assert_eq!(point!(x: coord.x(), y: coord.y()), actual);
}

#[test]
fn test_point_on_line() {
    let ruler = fixtures::ruler_km();

    // not Turf comparison because pointOnLine is bugged https://github.com/Turfjs/turf/issues/344
    let line = line_string![
        (x: -77.031669, y: 38.878605),
        (x: -77.029609, y: 38.881946),
    ];
    let point = point!(x: -77.034076, y: 38.882017);
    let result = ruler
        .point_on_line(&line, &point)
        .expect("Non-empty line string given");

    assert_eq_err!(-77.03052689033436, result.point().x(), 1e-6);
    assert_eq_err!(38.880457324462576, result.point().y(), 1e-6);
    assert_eq!(0, result.index()); // index
    assert_eq_err!(0.5544221677861756, result.t(), 1e-6); // t

    assert_eq!(
        0.0,
        ruler
            .point_on_line(&line, &point!(x: -80.0, y: 38.0))
            .expect("Non-empty line string given")
            .t(),
        "t is not less than 0"
    );
    assert_eq!(
        1.0,
        ruler
            .point_on_line(&line, &point!(x: -75.0, y: 38.0))
            .expect("Non-empty line string given")
            .t(),
        "t is not bigger than 1"
    );
}

#[test]
fn test_point_to_segment_distance() {
    let ruler = fixtures::ruler_km();

    let p = point!(x: -77.034076, y: 38.882017);
    let p0 = point!(x: -77.031669, y: 38.878605);
    let p1 = point!(x: -77.029609, y: 38.881946);

    let distance = ruler.point_to_segment_distance(&p, &p0, &p1);

    assert_eq_err!(0.37461484020420416, distance, 1e-6);
}

#[test]
fn test_line_slice() {
    let ruler = fixtures::ruler_km();

    for i in 0..LINES.len() {
        let dist = ruler.line_distance(&LINES[i]);
        let start = ruler
            .along(&LINES[i], dist * 0.3)
            .expect("Non-empty line string given");
        let stop = ruler
            .along(&LINES[i], dist * 0.7)
            .expect("Non-empty line string given");
        let expected = turf::LINESLICE[i];
        let actual =
            ruler.line_distance(&ruler.line_slice(&start, &stop, &LINES[i]));

        // @todo Should update turf_lineSlice and revert maxError back.
        assert_eq_err!(expected, actual, 1e-4);
    }
}

#[test]
fn test_line_slice_along_empty() {
    let ruler = fixtures::ruler_km();

    let empty_line = line_string![];
    let expected = ruler.line_distance(&empty_line);
    let actual =
        ruler.line_distance(&ruler.line_slice_along(0.0, 0.0, &empty_line));

    assert_eq_err!(expected, actual, 0.0);
}

#[test]
fn test_line_slice_along() {
    let ruler = fixtures::ruler_km();

    for i in 0..LINES.len() {
        if i == 46 {
            // skip due to Turf bug https://github.com/Turfjs/turf/issues/351
            //continue;
        }
        let dist = ruler.line_distance(&LINES[i]);
        let expected = turf::LINESLICE[i];
        let actual = ruler.line_distance(&ruler.line_slice_along(
            dist * 0.3,
            dist * 0.7,
            &LINES[i],
        ));

        // @todo Should update turf_lineSlice and revert maxError back.
        assert_eq_err!(expected, actual, 1e-4);
    }
}

#[test]
fn test_line_slice_reverse() {
    let ruler = fixtures::ruler_km();

    let line = &LINES[0];
    let dist = ruler.line_distance(line);
    let start = ruler
        .along(line, dist * 0.7)
        .expect("Non-empty line string given");
    let stop = ruler
        .along(line, dist * 0.3)
        .expect("Non-empty line string given");
    let actual = ruler.line_distance(&ruler.line_slice(&start, &stop, line));

    assert_eq_err!(0.018676476689649835, actual, 1e-6);
}

#[test]
fn test_buffer_point() {
    let ruler_miles = fixtures::ruler_miles();

    for i in 0..POINTS.len() {
        let (expected_min, expected_max) = turf::BUFFERPOINT[i];
        let actual = ruler_miles.buffer_point(&POINTS[i], 0.1);

        assert_eq_err!(expected_min.x(), actual.min().x, 2e-7);
        assert_eq_err!(expected_min.x(), actual.min().x, 2e-7);
        assert_eq_err!(expected_max.y(), actual.max().y, 2e-7);
        assert_eq_err!(expected_max.y(), actual.max().y, 2e-7);
    }
}

#[test]
fn test_buffer_bbox() {
    let ruler = fixtures::ruler_km();

    let bbox = Rect::new(
        Coordinate { x: 30.0, y: 38.0 },
        Coordinate { x: 40.0, y: 39.0 },
    );
    let bbox2 = ruler.buffer_bbox(&bbox, 1.0);

    assert_eq_err!(29.989319515875376, bbox2.min().x, 1e-6);
    assert_eq_err!(37.99098271225711, bbox2.min().y, 1e-6);
    assert_eq_err!(40.01068048412462, bbox2.max().x, 1e-6);
    assert_eq_err!(39.00901728774289, bbox2.max().y, 1e-6);
}

#[test]
fn test_inside_bbox() {
    let ruler = fixtures::ruler_km();

    let bbox = Rect::new(
        Coordinate { x: 30.0, y: 38.0 },
        Coordinate { x: 40.0, y: 39.0 },
    );

    assert!(ruler.inside_bbox(&point!(x: 35.0, y: 38.5), &bbox));
    assert!(!ruler.inside_bbox(&point!(x: 45.0, y: 45.0), &bbox));
}

#[test]
fn test_inside_bbox_over_dateline() {
    let ruler = fixtures::ruler_km();

    let bbox = Rect::new(
        Coordinate { x: 179.9, y: 32.7 },
        Coordinate { x: -179.9, y: 32.9 },
    );

    assert!(ruler.inside_bbox(&point!(x: 180.0, y: 32.8), &bbox));
}

#[test]
fn test_from_tile() {
    let ruler = CheapRuler::new(50.5, DistanceUnit::Kilometers);

    let tile_ruler = CheapRuler::from_tile(11041, 15, DistanceUnit::Kilometers);
    let p1 = point!(x: 30.5, y: 50.5);
    let p2 = point!(x: 30.51, y: 50.51);

    assert_eq_err!(
        ruler.distance(&p1, &p2),
        tile_ruler.distance(&p1, &p2),
        2e-5
    );
}
