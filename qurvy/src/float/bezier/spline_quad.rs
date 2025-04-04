use crate::float::bezier::spline::CADSpline;
use crate::float::math::line::Line;
use crate::float::math::point::Point;
use crate::int::bezier::spline_quad::IntQuadSpline;


#[derive(Debug, Clone)]
pub(crate) struct QuadSpline {
    pub(super) a: Point,
    pub(super) am: Point,
    pub(super) bm: Point,
    pub(super) b: Point,
}

impl CADSpline for QuadSpline {
    #[inline]
    fn start(&self) -> Point {
        self.a
    }
    #[inline]
    fn start_dir(&self) -> Point {
        (self.am - self.a).normalized()
    }
    #[inline]
    fn end_dir(&self) -> Point {
        (self.b - self.bm).normalized()
    }
    #[inline]
    fn end(&self) -> Point {
        self.b
    }

    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> Point {
        let l0 = Line::new(self.a, self.am);
        let l1 = Line::new(self.am, self.bm);
        let l2 = Line::new(self.bm, self.b);

        let p0 = l0.split_at(step, split_factor);
        let p1 = l1.split_at(step, split_factor);
        let p2 = l2.split_at(step, split_factor);

        let p10 = Line::new(p0, p1).split_at(step, split_factor);
        let p11 = Line::new(p1, p2).split_at(step, split_factor);

        Line::new(p10, p11).split_at(step, split_factor)
    }
}

impl From<&IntQuadSpline> for QuadSpline {
    fn from(value: &IntQuadSpline) -> Self {
        Self {
            a: value.a.into(),
            am: value.am.into(),
            b: value.b.into(),
            bm: value.bm.into(),
        }
    }
}