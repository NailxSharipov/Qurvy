use crate::float::bezier::spline::SplinePointsIter;
use crate::float::math::line::Line;
use crate::float::math::point::Point;
use crate::int::bezier::spline_cube::IntCubeSpline;

#[derive(Debug, Clone)]
pub(crate) struct CubeSpline {
    pub(crate) a: Point,
    pub(crate) m: Point,
    pub(crate) b: Point,
}

impl SplinePointsIter for CubeSpline {
    type ResourceIter<'a> = CubeSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> CubeSplinePointsIterator {
        CubeSplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct CubeSplinePointsIterator<'a> {
    spline: &'a CubeSpline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a> CubeSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: u32, start: bool, end: bool, spline: &'a CubeSpline) -> Self {
        let count = (1 << split_factor) + end as usize;
        let i = (!start) as usize;
        Self { i, count, split_factor, spline }
    }
}

impl<'a> Iterator for CubeSplinePointsIterator<'a> {
    type Item = Point;

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

impl From<&IntCubeSpline> for CubeSpline {
    fn from(value: &IntCubeSpline) -> Self {
        Self {
            a: value.a.into(),
            m: value.m.into(),
            b: value.b.into(),
        }
    }
}