use crate::int::bezier::spline::IntCADSpline;
use crate::int::math::point::IntPoint;

pub(crate) trait IntSplinePointsIter {
    type ResourceIter<'a>: Iterator<Item = IntPoint>
    where
        Self: 'a;

    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> Self::ResourceIter<'_>;
}

impl<Spline> IntSplinePointsIter for Spline
where
    Spline: IntCADSpline,
{
    type ResourceIter<'a>
        = IntSplinePointsIterator<'a, Spline>
    where
        Spline: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> IntSplinePointsIterator<Self> {
        IntSplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct IntSplinePointsIterator<'a, Spline> {
    spline: &'a Spline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a, Spline> IntSplinePointsIterator<'a, Spline> {
    #[inline]
    pub(crate) fn new(split_factor: u32, start: bool, end: bool, spline: &'a Spline) -> Self {
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

impl<'a, Spline: IntCADSpline> Iterator for IntSplinePointsIterator<'a, Spline> {
    type Item = IntPoint;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.count {
            return None;
        }

        let p = self.spline.split_at(self.i, self.split_factor);
        self.i += 1;

        Some(p)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}