extern crate cheap_ruler;
#[macro_use]
extern crate geo_types;

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
