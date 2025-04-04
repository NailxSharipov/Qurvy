use crate::convert::grid::Grid;
use crate::convert::to_float::ToFloat;
use crate::float::math::point::Point;
use crate::int::math::offset::IntOffset;
use serde::{Deserialize, Serialize};
use std::ops;

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
    pub fn normalized_10bit(&self) -> IntPoint {
        let dx = (self.x as i128).unsigned_abs().pow(2);
        let dy = (self.y as i128).unsigned_abs().pow(2);
        let sqr_len = dx + dy;
        let bits_count = sqr_len.ilog2();

        let len = sqr_len.isqrt() as i64;

        const VALUABLE_BITS: u32 = 10;
        const MAX_SAFE_BITS: u32 = 63 - VALUABLE_BITS;

        if bits_count <= MAX_SAFE_BITS {
            let x = (self.x << 10) / len;
            let y = (self.y << 10) / len;
            IntPoint::new(x, y)
        } else {
            let len = len >> 10;
            let x = self.x / len;
            let y = self.y / len;
            IntPoint::new(x, y)
        }
    }

    #[inline]
    pub fn dot_product(&self, other: &Self) -> i64 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross_product(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
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
    fn to_float(&self, grid: &Grid) -> Point {
        let x = grid.int_to_float(self.x);
        let y = grid.int_to_float(self.y);

        Point::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::IntPoint;
    use rand::Rng;

    #[test]
    fn test_basic_normalization() {
        assert_eq!(IntPoint::new(1024, 0).normalized_10bit(), IntPoint::new(1024, 0));
        assert_eq!(IntPoint::new(0, 1024).normalized_10bit(), IntPoint::new(0, 1024));
        assert_eq!(IntPoint::new(3, 4).normalized_10bit(), IntPoint::new(614, 819));
    }

    #[test]
    fn test_big_numbers() {
        let x: i64 = 507758875930;
        let y: i64 = 748317763344;
        let p = IntPoint::new(x, y).normalized_10bit();
        let n = p.normalized_10bit();
        assert!(n.x.abs() <= 1024);
        assert!(n.y.abs() <= 1024);

        let sqr_len = n.x * n.x + n.y * n.y;
        let error = 1024 * 1024 - sqr_len;

        assert!(error < 1024 * 100);
    }

    #[test]
    fn test_random_normalization_accuracy() {
        let mut rng = rand::rng();
        for _ in 0..1000 {
            let x = rng.random_range(-1_000_000_000_000..=1_000_000_000_000);
            let y = rng.random_range(-1_000_000_000_000..=1_000_000_000_000);
            if x == 0 && y == 0 {
                continue;
            }
            let p = IntPoint::new(x, y);
            let n = p.normalized_10bit();
            assert!(n.x.abs() <= 1024);
            assert!(n.y.abs() <= 1024);


            let sqr_len = n.x * n.x + n.y * n.y;
            let error = 1024 * 1024 - sqr_len;

            assert!(error < 1024 * 5);
        }
    }
}
