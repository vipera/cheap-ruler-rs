use geo_types::{CoordNum, Coordinate};
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct Rect<T: CoordNum> {
    min: Coordinate<T>,
    max: Coordinate<T>,
}

impl<T: CoordNum> Rect<T> {
    pub fn new<C>(c1: C, c2: C) -> Self
    where
        C: Into<Coordinate<T>>,
    {
        let min = c1.into();
        let max = c2.into();
        Self { min, max }
    }

    pub fn min(self) -> Coordinate<T> {
        self.min
    }

    pub fn max(self) -> Coordinate<T> {
        self.max
    }
}

impl<C, G> From<G> for Rect<C>
where
    C: CoordNum,
    G: Borrow<geo_types::Rect<C>>,
{
    fn from(geo_rect: G) -> Self {
        let geo_rect = geo_rect.borrow();
        Rect::new(geo_rect.min(), geo_rect.max())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rect_from_geo_types_rect() {
        let (min_x, max_x) = (30.0, 40.0);
        let (min_y, max_y) = (38.0, 39.0);

        // regular bounding box
        let bbox_regular = geo_types::Rect::new(
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
        );

        let bbox_rect_regular: Rect<f64> = bbox_regular.into();

        assert_eq!(bbox_rect_regular.min().x, min_x);
        assert_eq!(bbox_rect_regular.min().y, min_y);
        assert_eq!(bbox_rect_regular.max().x, max_x);
        assert_eq!(bbox_rect_regular.max().y, max_y);

        // bounding box crossing the international date line
        let (min_x, max_x) = (179.9, -179.9);
        let (min_y, max_y) = (32.7, 32.9);

        let bbox_date_line = geo_types::Rect::new(
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
        );

        let bbox_rect_date_line: Rect<f64> = bbox_date_line.into();

        // Note that min and max X are swapped
        assert_eq!(bbox_rect_date_line.min().x, max_x);
        assert_eq!(bbox_rect_date_line.min().y, min_y);
        assert_eq!(bbox_rect_date_line.max().x, min_x);
        assert_eq!(bbox_rect_date_line.max().y, max_y);
    }
}
