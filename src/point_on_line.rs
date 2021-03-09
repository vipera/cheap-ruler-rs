use geo_types::Point;
use num_traits::cast::NumCast;
use num_traits::Num;
use std::fmt;

pub struct PointOnLine<T>
where
    T: Num + NumCast + Copy + PartialEq + PartialOrd + fmt::Debug,
{
    point: Point<T>,
    index: usize,
    t: T,
}

impl<T> PointOnLine<T>
where
    T: Num + NumCast + Copy + PartialEq + PartialOrd + fmt::Debug,
{
    pub fn new(point: Point<T>, index: usize, t: T) -> Self {
        Self { point, index, t }
    }

    pub fn point(&self) -> Point<T> {
        self.point
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn t(&self) -> T {
        self.t
    }
}
