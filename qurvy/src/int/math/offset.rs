use serde::{Deserialize, Serialize};
use crate::convert::to_float::ToFloat;
use crate::float::math::offset::Offset;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntOffset {
    pub x: i64,
    pub y: i64,
}

impl IntOffset {
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    #[inline]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl From<IntPoint> for IntOffset {
    #[inline]
    fn from(value: IntPoint) -> Self {
        Self::new(value.x, value.y)
    }
}

impl ToFloat<Offset> for IntOffset {
    #[inline]
    fn to_float(&self, scale: f64) -> Offset {
        let x = scale * (self.x as f64);
        let y = scale * (self.y as f64);

        Offset::new(x, y)
    }
}