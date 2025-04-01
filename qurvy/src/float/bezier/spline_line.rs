use crate::float::bezier::spline::SplinePointsIter;
use crate::float::math::line::Line;
use crate::float::math::point::Point;
use crate::int::bezier::spline_line::IntLineSpline;

#[derive(Debug, Clone)]
pub(crate) struct LineSpline {
    pub(crate) a: Point,
    pub(crate) b: Point,
}

impl SplinePointsIter for LineSpline {
    type ResourceIter<'a> = LineSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> LineSplinePointsIterator {
        LineSplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct LineSplinePointsIterator<'a> {
    spline: &'a LineSpline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a> LineSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: u32, start: bool, end: bool, spline: &'a LineSpline) -> Self {
        let count = (1 << split_factor) + end as usize;
        let i = (!start) as usize;
        Self { i, count, split_factor, spline }
    }
}

impl<'a> Iterator for LineSplinePointsIterator<'a> {
    type Item = Point;

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

impl From<IntLineSpline> for LineSpline {
    fn from(value: IntLineSpline) -> Self {
        Self {
            a: value.a.into(),
            b: value.b.into(),
        }
    }
}