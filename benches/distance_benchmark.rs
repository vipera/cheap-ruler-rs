extern crate cheap_ruler;
#[macro_use]
extern crate criterion;
#[macro_use]
extern crate geo;
#[macro_use]
extern crate lazy_static;

use cheap_ruler::{CheapRuler, DistanceUnit};
use criterion::Criterion;
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

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_distance(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
