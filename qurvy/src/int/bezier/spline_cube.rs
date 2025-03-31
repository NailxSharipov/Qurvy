use crate::int::bezier::spline::SplinePointsIter;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;

pub(super) struct CubeSpline {
    pub(super) a: IntPoint,
    pub(super) m: IntPoint,
    pub(super) b: IntPoint,
}

impl SplinePointsIter for CubeSpline {
    type ResourceIter<'a> = CubeSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, exclude_last: bool, split_factor: usize) -> CubeSplinePointsIterator {
        CubeSplinePointsIterator::new(split_factor, exclude_last, self)
    }
}

pub(super) struct CubeSplinePointsIterator<'a> {
    spline: &'a CubeSpline,
    count: usize,
    split_factor: usize,
    i: usize,
}

impl<'a> CubeSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: usize, exclude_last: bool, spline: &'a CubeSpline) -> Self {
        let mut count = 1 << split_factor;
        if exclude_last {
            count -= 1;
        }
        Self { i: 0, count, split_factor, spline }
    }
}

impl<'a> Iterator for CubeSplinePointsIterator<'a> {
    type Item = IntPoint;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.count {
            return None;
        }

        let l0 = Line::new(self.spline.a, self.spline.m);
        let l1 = Line::new(self.spline.m, self.spline.b);
        let p10 = l0.split_at(self.i, self.split_factor);
        let p11 = l1.split_at(self.i, self.split_factor);
        let p = Line::new(p10, p11).split_at(self.i, self.split_factor);

        self.i += 1;

        Some(p)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}