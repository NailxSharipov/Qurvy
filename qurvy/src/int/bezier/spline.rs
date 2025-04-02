use crate::int::bezier::anchor::IntBezierAnchor;
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::bezier::spline_line::IntLineSpline;
use crate::int::bezier::spline_tetra::IntTetraSpline;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) enum IntSpline {
    Line(IntLineSpline),
    Cube(IntCubeSpline),
    Tetra(IntTetraSpline),
}

pub(crate) trait SplinePointsIter {
    type ResourceIter<'a>: Iterator<Item=IntPoint>
    where
        Self: 'a;

    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> Self::ResourceIter<'_>;
}

impl IntSpline {
    #[inline]
    pub(super) fn new(a: &IntBezierAnchor, b: &IntBezierAnchor) -> Self {
        match (a.handle_out_point(), b.handle_in_point()) {
            (Some(am), Some(bm)) => IntSpline::Tetra(IntTetraSpline {
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