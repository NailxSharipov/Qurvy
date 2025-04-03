use crate::int::bezier::spline::SplitAt;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) struct IntQuadraticSpline {
    pub(crate) a: IntPoint,
    pub(crate) am: IntPoint,
    pub(crate) bm: IntPoint,
    pub(crate) b: IntPoint,
}

impl SplitAt for IntQuadraticSpline {
    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint {
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