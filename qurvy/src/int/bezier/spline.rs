use crate::int::bezier::anchor::IntBezierAnchor;
use crate::int::bezier::approximation::IntApproximation;
use crate::int::bezier::iter::IntSplinePointsIter;
use crate::int::bezier::length::IntSplineLength;
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::bezier::spline_line::IntLineSpline;
use crate::int::bezier::spline_quad::IntQuadSpline;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) enum IntSpline {
    Line(IntLineSpline),
    Cube(IntCubeSpline),
    Quad(IntQuadSpline),
}

impl IntSpline {
    #[inline]
    pub(super) fn new(a: &IntBezierAnchor, b: &IntBezierAnchor) -> Self {
        match (a.handle_out_point(), b.handle_in_point()) {
            (Some(am), Some(bm)) => IntSpline::Quad(IntQuadSpline {
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
    pub fn regular_points(&self, split_factor: u32) -> Vec<IntPoint> {
        match self {
            IntSpline::Line(s) => s.points_iter(true, false, split_factor).collect(),
            IntSpline::Cube(s) => s.points_iter(true, false, split_factor).collect(),
            IntSpline::Quad(s) => s.points_iter(true, false, split_factor).collect(),
        }
    }

    #[inline]
    pub fn approximate_points(&self, min_cos: u32, min_len: u32) -> Vec<IntPoint> {
        match self {
            IntSpline::Line(s) => s.approximate_points(min_cos, min_len),
            IntSpline::Cube(s) => s.approximate_points(min_cos, min_len),
            IntSpline::Quad(s) => s.approximate_points(min_cos, min_len),
        }
    }

    #[inline]
    pub fn avg_length(&self, min_cos: u32, min_len: u32) -> u128 {
        match self {
            IntSpline::Line(s) => s.avg_length(min_cos, min_len),
            IntSpline::Cube(s) => s.avg_length(min_cos, min_len),
            IntSpline::Quad(s) => s.avg_length(min_cos, min_len),
        }
    }
}

pub(crate) trait IntCADSpline {
    fn start(&self) -> IntPoint;
    fn start_dir(&self) -> IntPoint;
    fn end_dir(&self) -> IntPoint;
    fn end(&self) -> IntPoint;
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint;
}
