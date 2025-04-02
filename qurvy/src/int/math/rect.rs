use crate::int::math::point::IntPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct IntRect {
    pub(crate) min: IntPoint,
    pub(crate) max: IntPoint,
}

impl IntRect {

    #[inline(always)]
    pub(crate) fn empty() -> Self {
        Self {
            min: IntPoint::new(i64::MAX, i64::MAX),
            max: IntPoint::new(i64::MIN, i64::MIN),
        }
    }

    #[inline(always)]
    pub(crate) fn with_min_max(min: IntPoint, max: IntPoint) -> Self {
        Self { min, max }
    }

    #[inline(always)]
    pub(crate) fn with_points(points: &[IntPoint]) -> Self {
        Self::with_iter(points.iter())
    }

    pub(crate) fn with_iter<'a, I: Iterator<Item=&'a IntPoint>>(iter: I) -> Self {
        let mut rect = Self::empty();

        for p in iter {
            rect.add_point(p);
        }

        rect
    }

    #[inline]
    pub(crate) fn add_point(&mut self, point: &IntPoint) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
    }

    #[inline]
    pub(crate) fn is_intersect_border_include(&self, other: &Self) -> bool {
        let x = self.min.x <= other.max.x && self.max.x >= other.min.x;
        let y = self.min.y <= other.max.y && self.max.y >= other.min.y;
        x && y
    }

    #[inline]
    pub(crate) fn is_intersect_border_exclude(&self, other: &Self) -> bool {
        let x = self.min.x < other.max.x && self.max.x > other.min.x;
        let y = self.min.y < other.max.y && self.max.y > other.min.y;
        x && y
    }
}

#[cfg(test)]
mod tests {
    use crate::int::math::point::IntPoint;
    use crate::int::math::rect::IntRect;

    #[test]
    fn test_0() {
        let rect = IntRect::with_points(
            &vec![
                IntPoint::new(0, 0),
                IntPoint::new(-7, 10),
                IntPoint::new(20, -5),
            ]
        );

        assert_eq!(rect.min.x, -7);
        assert_eq!(rect.max.x, 20);
        assert_eq!(rect.min.y, -5);
        assert_eq!(rect.max.y, 10);
    }

    #[test]
    fn test_1() {
        let a = IntRect::with_points(
            &vec![
                IntPoint::new(0, 0),
                IntPoint::new(10, 10)
            ]
        );

        let b = IntRect::with_points(
            &vec![
                IntPoint::new(10, 10),
                IntPoint::new(20, 0)
            ]
        );

        assert_eq!(a.is_intersect_border_exclude(&b), false);
        assert_eq!(a.is_intersect_border_include(&b), true);
    }
}
