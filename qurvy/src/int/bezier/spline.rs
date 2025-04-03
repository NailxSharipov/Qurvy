use crate::int::bezier::anchor::IntBezierAnchor;
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::bezier::spline_line::IntLineSpline;
use crate::int::bezier::spline_quadratic::IntQuadraticSpline;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) enum IntSpline {
    Line(IntLineSpline),
    Cube(IntCubeSpline),
    Tetra(IntQuadraticSpline),
}

impl IntSpline {
    #[inline]
    pub(super) fn new(a: &IntBezierAnchor, b: &IntBezierAnchor) -> Self {
        match (a.handle_out_point(), b.handle_in_point()) {
            (Some(am), Some(bm)) => IntSpline::Tetra(IntQuadraticSpline {
                a: a.point,
                am,
                bm,
                b: b.point,
            }),
            (Some(m), None) => IntSpline::Cube(IntCubeSpline {
                a: a.point,
                m,
                b: b.point,
            }),
            (None, Some(m)) => IntSpline::Cube(IntCubeSpline {
                a: a.point,
                m,
                b: b.point,
            }),
            (None, None) => IntSpline::Line(IntLineSpline {
                a: a.point,
                b: b.point,
            }),
        }
    }

    #[inline]
    pub fn fill(&self, target: &mut Vec<IntPoint>, split_factor: u32) {
        match self {
            IntSpline::Line(s) => target.extend(s.points_iter(true, false, split_factor)),
            IntSpline::Cube(s) => target.extend(s.points_iter(true, false, split_factor)),
            IntSpline::Tetra(s) => target.extend(s.points_iter(true, false, split_factor)),
        }
    }
}

pub(crate) trait SplitAt {
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint;
}

pub(crate) trait SplinePointsIter {
    type ResourceIter<'a>: Iterator<Item=IntPoint>
    where
        Self: 'a;

    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> Self::ResourceIter<'_>;
}

impl<T> SplinePointsIter for T
where
    T: SplitAt,
{
    type ResourceIter<'a> = SplinePointsIterator<'a, T>
    where
        T: 'a;

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

impl<'a, Spline: SplitAt> Iterator for SplinePointsIterator<'a, Spline> {
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