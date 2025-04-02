use crate::int::bezier::spline::SplinePointsIter;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;
use crate::int::math::rect::IntRect;

#[derive(Debug, Clone)]
pub(crate) struct IntLineSpline {
    pub(crate) a: IntPoint,
    pub(crate) b: IntPoint,
}

impl IntLineSpline {

    #[inline]
    pub(crate) fn boundary(&self) -> IntRect {
        let (min_x, max_x) = if self.a.x < self.b.x {
            (self.a.x, self.b.x)
        } else {
            (self.b.x, self.a.x)
        };
        let (min_y, max_y) = if self.a.y < self.b.y {
            (self.a.y, self.b.y)
        } else {
            (self.b.y, self.a.y)
        };
        IntRect {
            min: IntPoint { x: min_x, y: min_y },
            max: IntPoint { x: max_x, y: max_y },
        }
    }
}

impl SplinePointsIter for IntLineSpline {
    type ResourceIter<'a>
        = LineSplinePointsIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> LineSplinePointsIterator {
        LineSplinePointsIterator::new(split_factor, start, end, self)
    }
}

pub(crate) struct LineSplinePointsIterator<'a> {
    spline: &'a IntLineSpline,
    count: usize,
    split_factor: u32,
    i: usize,
}

impl<'a> LineSplinePointsIterator<'a> {
    #[inline]
    fn new(split_factor: u32, start: bool, end: bool, spline: &'a IntLineSpline) -> Self {
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
