use num_traits::Float;

/// Defines common units of distance that can be used
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_unit_conversions() {
        assert_eq!(value_1000(DistanceUnit::Kilometers), 1000);
        assert_eq!(value_1000(DistanceUnit::Miles), 621);
        assert_eq!(value_1000(DistanceUnit::NauticalMiles), 540);
        assert_eq!(value_1000(DistanceUnit::Meters), 1000000);
        assert_eq!(value_1000(DistanceUnit::Yards), 1093613);
        assert_eq!(value_1000(DistanceUnit::Feet), 3280840);
        assert_eq!(value_1000(DistanceUnit::Inches), 39370079);
    }

    fn value_1000(unit: DistanceUnit) -> i32 {
        (unit.conversion_factor_kilometers::<f64>() * 1000.0).round() as i32
    }
}
