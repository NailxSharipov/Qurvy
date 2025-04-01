use crate::float::bezier::spline::SplinePointsIter;
use crate::float::math::line::Line;
use crate::float::math::point::Point;
use crate::int::bezier::spline_tetra::IntTetraSpline;

#[derive(Debug, Clone)]
pub(crate) struct TetraSpline {
    pub(super) a: Point,
    pub(super) am: Point,
    pub(super) bm: Point,
    pub(super) b: Point,
}

impl SplinePointsIter for TetraSpline {
    type ResourceIter<'a> = TetraSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> TetraSplinePointsIterator {
        TetraSplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct TetraSplinePointsIterator<'a> {
    spline: &'a TetraSpline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a> TetraSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: u32, start: bool, end: bool, spline: &'a TetraSpline) -> Self {
        let count = (1 << split_factor) + end as usize;
        let i = (!start) as usize;
        Self { i, count, split_factor, spline }
    }
}

impl<'a> Iterator for TetraSplinePointsIterator<'a> {
    type Item = Point;

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

impl From<&IntTetraSpline> for TetraSpline {
    fn from(value: &IntTetraSpline) -> Self {
        Self {
            a: value.a.into(),
            am: value.am.into(),
            b: value.b.into(),
            bm: value.bm.into(),
        }
    }
}