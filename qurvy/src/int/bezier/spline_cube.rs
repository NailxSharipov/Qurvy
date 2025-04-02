use crate::int::bezier::spline::SplinePointsIter;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) struct IntCubeSpline {
    pub(crate) a: IntPoint,
    pub(crate) m: IntPoint,
    pub(crate) b: IntPoint,
}

impl SplinePointsIter for IntCubeSpline {
    type ResourceIter<'a>
        = CubeSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> CubeSplinePointsIterator {
        CubeSplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct CubeSplinePointsIterator<'a> {
    spline: &'a IntCubeSpline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a> CubeSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: u32, start: bool, end: bool, spline: &'a IntCubeSpline) -> Self {
        let count = (1 << split_factor) + end as usize;
        let i = (!start) as usize;
        Self {
            i,
            count,
            split_factor,
            spline,
        }
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
