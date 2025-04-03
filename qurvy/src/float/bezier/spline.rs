use crate::float::bezier::anchor::BezierAnchor;
use crate::float::bezier::spline_cube::CubeSpline;
use crate::float::bezier::spline_line::LineSpline;
use crate::float::bezier::spline_quadratic::QuadraticSpline;
use crate::float::math::point::Point;

#[derive(Debug, Clone)]
pub(crate) enum Spline {
    Line(LineSpline),
    Cube(CubeSpline),
    Tetra(QuadraticSpline),
}

pub(crate) trait SplinePointsIter {
    type ResourceIter<'a>: Iterator<Item=Point>
    where
        Self: 'a;

    fn points_iter(&self, start: bool, end: bool, split_factor: u32) -> Self::ResourceIter<'_>;
}

impl Spline {
    #[inline]
    pub(super) fn new(a: &BezierAnchor, b: &BezierAnchor) -> Self {
        match (a.handle_out_point(), b.handle_in_point()) {
            (Some(am), Some(bm)) => Spline::Tetra(QuadraticSpline {
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
}