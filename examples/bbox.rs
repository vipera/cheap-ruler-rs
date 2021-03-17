extern crate cheap_ruler;
#[macro_use]
extern crate geo_types;

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
