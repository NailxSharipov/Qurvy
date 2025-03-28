use std::ops;
use serde::{Deserialize, Serialize};
use crate::convert::to_int::ToInt;
use crate::float::math::offset::Offset;
use crate::int::math::offset::IntOffset;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<Offset> for Point {
    #[inline]
    fn from(value: Offset) -> Self {
        Self::new(value.x, value.y)
    }
}

impl ToInt<IntPoint> for Point {
    #[inline]
    fn to_int(&self, scale: f64) -> IntPoint {
        let x = (scale * self.x) as i64;
        let y = (scale * self.y) as i64;

        IntPoint::new(x, y)
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