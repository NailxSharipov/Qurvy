use std::ops;
use serde::{Deserialize, Serialize};
use crate::convert::to_float::ToFloat;
use crate::float::math::point::Point;
use crate::int::math::offset::IntOffset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntPoint {
    pub x: i64,
    pub y: i64,
}

impl IntPoint {
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    #[inline]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn mid(&self, other: &Self) -> Self {
        let x = (self.x + other.x) / 2;
        let y = (self.y + other.y) / 2;

        Self { x, y }
    }
}


impl ops::Add for IntPoint {
    type Output = IntPoint;

    #[inline(always)]
    fn add(self, other: IntPoint) -> IntPoint {
        IntPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Add<IntOffset> for IntPoint {
    type Output = IntPoint;

    #[inline(always)]
    fn add(self, other: IntOffset) -> IntPoint {
        IntPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for IntPoint {
    type Output = IntPoint;

    #[inline(always)]
    fn sub(self, other: IntPoint) -> IntPoint {
        IntPoint {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Sub<IntOffset> for IntPoint {
    type Output = IntPoint;

    #[inline(always)]
    fn sub(self, other: IntOffset) -> IntPoint {
        IntPoint {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<IntOffset> for IntPoint {
    #[inline]
    fn from(value: IntOffset) -> Self {
        Self::new(value.x, value.y)
    }
}

impl ToFloat<Point> for IntPoint {
    #[inline]
    fn to_float(&self, scale: f64) -> Point {
        let x = scale * (self.x as f64);
        let y = scale * (self.y as f64);

        Point::new(x, y)
    }
}
