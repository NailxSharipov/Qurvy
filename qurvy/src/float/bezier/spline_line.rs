use crate::float::bezier::spline::CADSpline;
use crate::float::math::line::Line;
use crate::float::math::point::Point;
use crate::int::bezier::spline_line::IntLineSpline;

#[derive(Debug, Clone)]
pub(crate) struct LineSpline {
    pub(crate) a: Point,
    pub(crate) b: Point,
}

impl CADSpline for LineSpline {
    #[inline]
    fn start(&self) -> Point {
        self.a
    }
    #[inline]
    fn start_dir(&self) -> Point {
        (self.b - self.a).normalized()
    }

    #[inline]
    fn end_dir(&self) -> Point {
        (self.b - self.a).normalized()
    }
    #[inline]
    fn end(&self) -> Point {
        self.b
    }

    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> Point {
        Line::new(self.a, self.b).split_at(step, split_factor)
    }
}


impl From<IntLineSpline> for LineSpline {
    fn from(value: IntLineSpline) -> Self {
        Self {
            a: value.a.into(),
            b: value.b.into(),
        }
    }
}