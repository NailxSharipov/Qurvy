use crate::float::bezier::spline::CADSpline;
use crate::float::math::point::Point;

pub(crate) trait SplinePointsIter {
    type ResourceIter<'a>: Iterator<Item = Point>
    where
        Self: 'a;

    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> Self::ResourceIter<'_>;
}

impl<Spline> SplinePointsIter for Spline
where
    Spline: CADSpline,
{
    type ResourceIter<'a> = SplinePointsIterator<'a, Spline>
    where
        Spline: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> SplinePointsIterator<Self> {
        SplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct SplinePointsIterator<'a, Spline> {
    spline: &'a Spline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a, Spline> SplinePointsIterator<'a, Spline> {
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

impl<'a, Spline: CADSpline> Iterator for SplinePointsIterator<'a, Spline> {
    type Item = Point;

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