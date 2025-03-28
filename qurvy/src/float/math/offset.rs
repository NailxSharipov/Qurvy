use serde::{Deserialize, Serialize};
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
    fn to_int(&self, scale: f64) -> IntOffset {
        let x = (scale * self.x) as i64;
        let y = (scale * self.y) as i64;

        IntOffset::new(x, y)
    }
}