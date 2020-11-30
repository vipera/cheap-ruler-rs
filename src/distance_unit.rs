/// Defines common units of distance that can be used
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DistanceUnit {
    Kilometers,
    Miles,
    NauticalMiles,
    Meters,
    Yards,
    Feet,
    Inches,
}

impl DistanceUnit {
    /// Provides a factor that scales the unit into kilometers
    pub(crate) fn conversion_factor_kilometers(&self) -> f64 {
        match *self {
            DistanceUnit::Kilometers => 1.0,
            DistanceUnit::Miles => 1000.0 / 1609.344,
            DistanceUnit::NauticalMiles => 1000.0 / 1852.0,
            DistanceUnit::Meters => 1000.0,
            DistanceUnit::Yards => 1000.0 / 0.9144,
            DistanceUnit::Feet => 1000.0 / 0.3048,
            DistanceUnit::Inches => 1000.0 / 0.0254,
        }
    }
}
