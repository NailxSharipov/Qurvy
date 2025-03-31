use crate::int::bezier::spline::SplinePointsIter;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;

pub(super) struct LineSpline {
    pub(super) a: IntPoint,
    pub(super) b: IntPoint,
}

impl SplinePointsIter for LineSpline {
    type ResourceIter<'a> = LineSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, exclude_last: bool, split_factor: usize) -> LineSplinePointsIterator {
        LineSplinePointsIterator::new(split_factor, exclude_last, self)
    }
}

pub(super) struct LineSplinePointsIterator<'a> {
    spline: &'a LineSpline,
    count: usize,
    split_factor: usize,
    i: usize,
}

impl<'a> LineSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: usize, exclude_last: bool, spline: &'a LineSpline) -> Self {
        let mut count = 1 << split_factor;
        if exclude_last {
            count -= 1;
        }
        Self { i: 0, count, split_factor, spline }
    }
}

impl<'a> Iterator for LineSplinePointsIterator<'a> {
    type Item = IntPoint;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.count {
            return None;
        }
        let p = Line::new(self.spline.a, self.spline.b).split_at(self.i, self.split_factor);

        self.i += 1;

        Some(p)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}