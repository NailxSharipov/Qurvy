use crate::float::bezier::spline::CADSpline;
use crate::float::math::line::Line;
use crate::float::math::point::Point;
use crate::int::bezier::spline_cube::IntCubeSpline;

#[derive(Debug, Clone)]
pub(crate) struct CubeSpline {
    pub(crate) a: Point,
    pub(crate) m: Point,
    pub(crate) b: Point,
}

impl CADSpline for CubeSpline {
    #[inline]
    fn start(&self) -> Point {
        self.a
    }
    #[inline]
    fn start_dir(&self) -> Point {
        (self.m - self.a).normalized()
    }
    #[inline]
    fn end_dir(&self) -> Point {
        (self.b - self.m).normalized()
    }
    #[inline]
    fn end(&self) -> Point {
        self.b
    }

    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> Point {
        let l0 = Line::new(self.a, self.m);
        let l1 = Line::new(self.m, self.b);
        let p10 = l0.split_at(step, split_factor);
        let p11 = l1.split_at(step, split_factor);
        Line::new(p10, p11).split_at(step, split_factor)
    }
}

impl From<&IntCubeSpline> for CubeSpline {
    fn from(value: &IntCubeSpline) -> Self {
        Self {
            a: value.a.into(),
            m: value.m.into(),
            b: value.b.into(),
        }
    }
}