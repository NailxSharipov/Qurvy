use crate::int::bezier::anchor::IntBezierAnchor;
use crate::int::bezier::spline_cube::CubeSpline;
use crate::int::bezier::spline_line::LineSpline;
use crate::int::bezier::spline_tetra::TetraSpline;
use crate::int::math::point::IntPoint;

pub(super) enum Spline {
    Line(LineSpline),
    Cube(CubeSpline),
    Tetra(TetraSpline),
}

pub(super) trait SplinePointsIter {
    type ResourceIter<'a>: Iterator<Item=IntPoint>
    where
        Self: 'a;

    fn points_iter(&self, exclude_last: bool, split_factor: usize) -> Self::ResourceIter<'_>;
}

impl Spline {
    #[inline]
    pub(super) fn new(a: &IntBezierAnchor, b: &IntBezierAnchor) -> Self {
        match (a.handle_out_point(), b.handle_in_point()) {
            (Some(am), Some(bm)) => Spline::Tetra(TetraSpline {
                a: a.point,
                am,
                bm,
                b: b.point,
            }),
            (Some(m), None) => Spline::Cube(CubeSpline {
                a: a.point,
                m,
                b: b.point,
            }),
            (None, Some(m)) => Spline::Cube(CubeSpline {
                a: a.point,
                m,
                b: b.point,
            }),
            (None, None) => Spline::Line(LineSpline {
                a: a.point,
                b: b.point,
            }),
        }
    }

    #[inline]
    pub fn fill(&self, target: &mut Vec<IntPoint>, exclude_last: bool, split_factor: usize) {
        match self {
            Spline::Line(s) => target.extend(s.points_iter(exclude_last, split_factor)),
            Spline::Cube(s) => target.extend(s.points_iter(exclude_last, split_factor)),
            Spline::Tetra(s) => target.extend(s.points_iter(exclude_last, split_factor)),
        }
    }
}