use crate::float::bezier::anchor::BezierAnchor;
use crate::float::bezier::approximation::Approximation;
use crate::float::bezier::iter::SplinePointsIter;
use crate::float::bezier::spline_cube::CubeSpline;
use crate::float::bezier::spline_line::LineSpline;
use crate::float::bezier::spline_quad::QuadSpline;
use crate::float::math::length::SplineLength;
use crate::float::math::point::Point;

#[derive(Debug, Clone)]
pub(crate) enum Spline {
    Line(LineSpline),
    Cube(CubeSpline),
    Quad(QuadSpline),
}

impl Spline {
    #[inline]
    pub(super) fn new(a: &BezierAnchor, b: &BezierAnchor) -> Self {
        match (a.handle_out_point(), b.handle_in_point()) {
            (Some(am), Some(bm)) => Spline::Quad(QuadSpline {
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
    pub fn regular_points(&self, split_factor: u32) -> Vec<Point> {
        match self {
            Spline::Line(s) => s.points_iter(true, false, split_factor).collect(),
            Spline::Cube(s) => s.points_iter(true, false, split_factor).collect(),
            Spline::Quad(s) => s.points_iter(true, false, split_factor).collect(),
        }
    }

    #[inline]
    pub fn approximate_points(&self, min_cos: f64, min_len: f64) -> Vec<Point> {
        match self {
            Spline::Line(s) => s.approximate_points(min_cos, min_len),
            Spline::Cube(s) => s.approximate_points(min_cos, min_len),
            Spline::Quad(s) => s.approximate_points(min_cos, min_len),
        }
    }

    #[inline]
    pub fn avg_length(&self, min_cos: f64, min_len: f64) -> f64 {
        match self {
            Spline::Line(s) => s.avg_length(min_cos, min_len),
            Spline::Cube(s) => s.avg_length(min_cos, min_len),
            Spline::Quad(s) => s.avg_length(min_cos, min_len),
        }
    }
}

pub(crate) trait CADSpline {
    fn start(&self) -> Point;
    fn start_dir(&self) -> Point;
    fn end_dir(&self) -> Point;
    fn end(&self) -> Point;
    fn split_at(&self, step: usize, split_factor: u32) -> Point;
}