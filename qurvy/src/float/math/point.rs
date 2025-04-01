use std::ops;
use serde::{Deserialize, Serialize};
use crate::convert::to_int::ToInt;
use crate::float::math::offset::Offset;
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

    #[inline]
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }
}

impl From<Offset> for Point {
    #[inline]
    fn from(value: Offset) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<IntPoint> for Point {
    #[inline]
    fn from(value: IntPoint) -> Self {
        Self::new(value.x as f64, value.y as f64)
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

impl ops::Add for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Add<Offset> for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Offset) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Sub<Offset> for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Offset) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}