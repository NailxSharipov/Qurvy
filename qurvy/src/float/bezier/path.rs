use serde::{Deserialize, Serialize};
use crate::convert::grid::Grid;
use crate::convert::to_int::ToInt;
use crate::float::bezier::anchor::BezierAnchor;
use crate::float::bezier::spline::Spline;
use crate::float::math::point::Point;
use crate::int::bezier::path::IntBezierPath;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BezierPath {
    pub anchors: Vec<BezierAnchor>,
    pub closed: bool,
}

impl BezierPath {
    #[inline]
    pub fn regular_points(&self, split_factor: u32) -> Vec<Point> {
        let capacity = self.anchors.len() << split_factor;
        let mut points = Vec::with_capacity(capacity);
        for spline in self.splines() {
            points.append(&mut spline.regular_points(split_factor));
        }

        points
    }

    #[inline]
    pub(crate) fn splines(&self) -> impl Iterator<Item = Spline> + '_ {
        SplineIterator::new(self)
    }
}

pub(crate) struct SplineIterator<'a> {
    path: &'a BezierPath,
    i: usize,
}

impl<'a> SplineIterator<'a> {
    #[inline]
    fn new(path: &'a BezierPath) -> Self {
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

impl ToInt<IntBezierPath> for BezierPath {
    #[inline]
    fn to_int(&self, grid: &Grid) -> IntBezierPath {
        IntBezierPath {
            anchors: self.anchors.iter().map(|a|a.to_int(grid)).collect(),
            closed: self.closed,
        }
    }
}