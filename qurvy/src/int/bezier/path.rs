use crate::convert::to_float::ToFloat;
use crate::float::bezier::path::BezierPath;
use crate::int::bezier::anchor::IntBezierAnchor;
use crate::int::bezier::spline::Spline;
use crate::int::math::point::IntPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntBezierPath {
    pub anchors: Vec<IntBezierAnchor>,
    pub closed: bool,
}

impl ToFloat<BezierPath> for IntBezierPath {
    #[inline]
    fn to_float(&self, scale: f64) -> BezierPath {
        BezierPath {
            anchors: self.anchors.iter().map(|a| a.to_float(scale)).collect(),
            closed: self.closed,
        }
    }
}

impl IntBezierPath {
    #[inline]
    pub fn points(&self, split_factor: usize) -> Vec<IntPoint> {
        let capacity = self.anchors.len() << split_factor;
        let mut points = Vec::with_capacity(capacity);
        for spline in self.splines() {
            spline.fill(&mut points, true, split_factor);
        }

        points
    }

    #[inline]
    fn splines(&self) -> impl Iterator<Item = Spline> + '_ {
        SplineIterator::new(self)
    }
}

struct SplineIterator<'a> {
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
    type Item = Spline;

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
                Some(Spline::new(last, first))
            } else {
                None
            };
        }

        let a0 = &self.path.anchors[self.i - 1];
        let a1 = &self.path.anchors[self.i];

        self.i += 1;

        Some(Spline::new(a0, a1))
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
                }
            ],
            closed: true,
        };

        let points = path.points(2);

        assert_eq!(points.len(), 16);
    }
}