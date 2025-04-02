use crate::convert::to_float::ToFloat;
use crate::float::bezier::path::BezierPath;
use crate::int::bezier::anchor::IntBezierAnchor;
use crate::int::bezier::spline::IntSpline;
use crate::int::math::point::IntPoint;
use serde::{Deserialize, Serialize};
use crate::convert::grid::Grid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntBezierPath {
    pub anchors: Vec<IntBezierAnchor>,
    pub closed: bool,
}

impl ToFloat<BezierPath> for IntBezierPath {
    #[inline]
    fn to_float(&self, grid: &Grid) -> BezierPath {
        BezierPath {
            anchors: self.anchors.iter().map(|a| a.to_float(grid)).collect(),
            closed: self.closed,
        }
    }
}

impl IntBezierPath {
    #[inline]
    pub fn points(&self, split_factor: u32) -> Vec<IntPoint> {
        let capacity = self.anchors.len() << split_factor;
        let mut points = Vec::with_capacity(capacity);
        for spline in self.splines() {
            spline.fill(&mut points, split_factor);
        }

        points
    }

    #[inline]
    pub(crate) fn splines(&self) -> impl Iterator<Item = IntSpline> + '_ {
        SplineIterator::new(self)
    }
}

pub(crate) struct SplineIterator<'a> {
    path: &'a IntBezierPath,
    i: usize,
}

impl<'a> SplineIterator<'a> {
    #[inline]
    fn new(path: &'a IntBezierPath) -> Self {
        Self { i: 1, path }
    }
}

impl<'a> Iterator for SplineIterator<'a> {
    type Item = IntSpline;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.path.anchors.len() {
            return None;
        }

        if self.i == self.path.anchors.len() {
            self.i += 1;
            return if self.path.closed {
                let first = self.path.anchors.first().unwrap();
                let last = self.path.anchors.last().unwrap();
                Some(IntSpline::new(last, first))
            } else {
                None
            };
        }

        let a0 = &self.path.anchors[self.i - 1];
        let a1 = &self.path.anchors[self.i];

        self.i += 1;

        Some(IntSpline::new(a0, a1))
    }
}

#[cfg(test)]
mod tests {
    use crate::int::bezier::anchor::IntBezierAnchor;
    use crate::int::bezier::path::IntBezierPath;
    use crate::int::math::offset::IntOffset;
    use crate::int::math::point::IntPoint;

    #[test]
    fn test_00() {
        let path = IntBezierPath {
            anchors: vec![
                IntBezierAnchor {
                    point: IntPoint { x: -1000, y: 0 },
                    handle_in: Some(IntOffset { x: 0, y: -100 }),
                    handle_out: Some(IntOffset { x: 0, y: 100 }),
                },
                IntBezierAnchor {
                    point: IntPoint { x: 0, y: 1000 },
                    handle_in: Some(IntOffset { x: -100, y: 0 }),
                    handle_out: Some(IntOffset { x: 100, y: 0 }),
                },
                IntBezierAnchor {
                    point: IntPoint { x: 1000, y: 0 },
                    handle_in: Some(IntOffset { x: 0, y: 100 }),
                    handle_out: Some(IntOffset { x: 0, y: -100 }),
                },
                IntBezierAnchor {
                    point: IntPoint { x: 0, y: -1000 },
                    handle_in: Some(IntOffset { x: 100, y: 0 }),
                    handle_out: Some(IntOffset { x: -100, y: 0 }),
                },
            ],
            closed: true,
        };

        let points = path.points(2);

        assert_eq!(points.len(), 16);
    }
}
