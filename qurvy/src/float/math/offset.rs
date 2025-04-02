use serde::{Deserialize, Serialize};
use crate::convert::grid::Grid;
use crate::convert::to_int::ToInt;
use crate::float::math::point::Point;
use crate::int::math::offset::IntOffset;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Offset {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<Point> for Offset {
    #[inline]
    fn from(value: Point) -> Self {
        Self::new(value.x, value.y)
    }
}

impl ToInt<IntOffset> for Offset {
    #[inline]
    fn to_int(&self, grid: &Grid) -> IntOffset {
        let x = grid.float_to_int(self.x);
        let y = grid.float_to_int(self.y);

        IntOffset::new(x, y)
    }
}