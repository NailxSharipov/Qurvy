use std::ops;
use serde::{Deserialize, Serialize};
use crate::convert::grid::Grid;
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
    pub fn sqr_length(&self) -> f64 {
        let dx = self.x;
        let dy = self.y;

        dx * dx + dy * dy
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.sqr_length().sqrt()
    }

    #[inline]
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Point {
        let inv_len = 1.0 / self.length();
        let x = self.x * inv_len;
        let y = self.y * inv_len;
        Point { x, y }
    }

    #[inline]
    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
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

impl ToInt<IntPoint> for Point {
    #[inline]
    fn to_int(&self, grid: &Grid) -> IntPoint {
        let x = grid.float_to_int(self.x);
        let y = grid.float_to_int(self.y);

        IntPoint::new(x, y)
    }
}