use crate::int::bezier::spline::SplinePointsIter;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;

pub(super) struct TetraSpline {
    pub(super) a: IntPoint,
    pub(super) am: IntPoint,
    pub(super) bm: IntPoint,
    pub(super) b: IntPoint,
}

impl SplinePointsIter for TetraSpline {
    type ResourceIter<'a> = TetraSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, exclude_last: bool, split_factor: usize) -> TetraSplinePointsIterator {
        TetraSplinePointsIterator::new(split_factor, exclude_last, self)
    }
}

pub(super) struct TetraSplinePointsIterator<'a> {
    spline: &'a TetraSpline,
    count: usize,
    split_factor: usize,
    i: usize,
}

impl<'a> TetraSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: usize, exclude_last: bool, spline: &'a TetraSpline) -> Self {
        let mut count = 1 << split_factor;
        if exclude_last {
            count -= 1;
        }
        Self { i: 0, count, split_factor, spline }
    }
}

impl<'a> Iterator for TetraSplinePointsIterator<'a> {
    type Item = IntPoint;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.count {
            return None;
        }

        let l0 = Line::new(self.spline.a, self.spline.am);
        let l1 = Line::new(self.spline.am, self.spline.bm);
        let l2 = Line::new(self.spline.bm, self.spline.b);

        let p0 = l0.split_at(self.i, self.split_factor);
        let p1 = l1.split_at(self.i, self.split_factor);
        let p2 = l2.split_at(self.i, self.split_factor);

        let p10 = Line::new(p0, p1).split_at(self.i, self.split_factor);
        let p11 = Line::new(p1, p2).split_at(self.i, self.split_factor);

        let p = Line::new(p10, p11).split_at(self.i, self.split_factor);

        self.i += 1;

        Some(p)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}