extern crate cheap_ruler;
use cheap_ruler::{CheapRuler, DistanceUnit};

fn main() {
    let mut ruler = CheapRuler::new(44.7192003, DistanceUnit::Meters);
    println!("Distance unit: {:?}", ruler.distance_unit());

    ruler.change_unit(DistanceUnit::Miles);
    println!("Distance unit: {:?}", ruler.distance_unit());
}
