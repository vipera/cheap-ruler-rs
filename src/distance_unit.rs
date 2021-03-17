use num_traits::Float;

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
    pub(crate) fn conversion_factor_kilometers<T: Float>(&self) -> T {
        match *self {
            DistanceUnit::Kilometers => T::one(),
            DistanceUnit::Miles => T::from(1000f64 / 1609.344).unwrap(),
            DistanceUnit::NauticalMiles => T::from(1000f64 / 1852.0).unwrap(),
            DistanceUnit::Meters => T::from(1000f64).unwrap(),
            DistanceUnit::Yards => T::from(1000f64 / 0.9144).unwrap(),
            DistanceUnit::Feet => T::from(1000f64 / 0.3048).unwrap(),
            DistanceUnit::Inches => T::from(1000f64 / 0.0254).unwrap(),
        }
    }
}
