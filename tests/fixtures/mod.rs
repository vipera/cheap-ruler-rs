use cheap_ruler::{CheapRuler, DistanceUnit};

static CR_LATITUDE: f64 = 32.8351;

pub(super) mod lines;
pub(super) mod turf;

pub(super) fn ruler_km() -> CheapRuler {
    CheapRuler::new(CR_LATITUDE, DistanceUnit::Kilometers)
}

pub(super) fn ruler_miles() -> CheapRuler {
    CheapRuler::new(CR_LATITUDE, DistanceUnit::Miles)
}
