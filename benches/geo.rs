extern crate cheap_ruler;
#[macro_use]
extern crate criterion;
#[macro_use]
extern crate geo;
#[macro_use]
extern crate lazy_static;

use cheap_ruler::{CheapRuler, DistanceUnit};
use criterion::Criterion;
use geo::algorithm::bearing::Bearing;
use geo::algorithm::haversine_destination::HaversineDestination;
use geo::algorithm::haversine_distance::HaversineDistance;
use geo::algorithm::vincenty_distance::VincentyDistance;
use geo::Point;

lazy_static! {
    static ref STINICA: Point<f64> = point!(x: 14.890127, y: 44.7195996);
    static ref MISNJAK: Point<f64> = point!(x: 14.8609779, y: 44.7061625);
}

fn cheap_ruler_distance(from: &Point<f64>, to: &Point<f64>) -> f64 {
    let ruler = CheapRuler::new(from.lat(), DistanceUnit::Meters);
    ruler.distance(from, to)
}

fn haversine_distance(from: &Point<f64>, to: &Point<f64>) -> f64 {
    from.haversine_distance(to)
}

fn vincenty_distance(from: &Point<f64>, to: &Point<f64>) -> f64 {
    from.vincenty_distance(to).unwrap()
}

fn benchmark_distance(c: &mut Criterion) {
    let input = (STINICA.clone(), MISNJAK.clone());

    let mut group = c.benchmark_group("distance");

    group.bench_with_input("cheap_ruler", &input, |b, &(from, to)| {
        b.iter(|| cheap_ruler_distance(&from, &to))
    });

    group.bench_with_input("haversine", &input, |b, &(from, to)| {
        b.iter(|| haversine_distance(&from, &to))
    });

    group.bench_with_input("vincenty", &input, |b, &(from, to)| {
        b.iter(|| vincenty_distance(&from, &to))
    });

    group.finish();
}

fn cheap_ruler_bearing(from: &Point<f64>, to: &Point<f64>) -> f64 {
    let ruler = CheapRuler::new(from.lat(), DistanceUnit::Meters);
    ruler.bearing(from, to)
}

fn geo_bearing(from: &Point<f64>, to: &Point<f64>) -> f64 {
    from.bearing(to.to_owned())
}

fn benchmark_bearing(c: &mut Criterion) {
    let input = (STINICA.clone(), MISNJAK.clone());

    let mut group = c.benchmark_group("bearing");

    group.bench_with_input("cheap_ruler", &input, |b, &(from, to)| {
        b.iter(|| cheap_ruler_bearing(&from, &to))
    });

    group.bench_with_input("geo", &input, |b, &(from, to)| {
        b.iter(|| geo_bearing(&from, &to))
    });

    group.finish();
}

fn cheap_ruler_destination(
    bearing: f64,
    dist: f64,
    from: &Point<f64>,
) -> Point<f64> {
    let ruler = CheapRuler::new(from.lat(), DistanceUnit::Meters);
    ruler.destination(from, dist, bearing)
}

fn haversine_destination(
    bearing: f64,
    dist: f64,
    from: &Point<f64>,
) -> Point<f64> {
    from.haversine_destination(bearing, dist)
}

fn benchmark_destination(c: &mut Criterion) {
    let input = (45.0, 1000., STINICA.clone());

    let mut group = c.benchmark_group("destination");

    group.bench_with_input(
        "cheap_ruler",
        &input,
        |b, &(bearing, dist, from)| {
            b.iter(|| cheap_ruler_destination(bearing, dist, &from))
        },
    );

    group.bench_with_input("haversine", &input, |b, &(bearing, dist, from)| {
        b.iter(|| haversine_destination(bearing, dist, &from))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_distance,
    benchmark_bearing,
    benchmark_destination,
);
criterion_main!(benches);
