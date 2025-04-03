use crate::int::bezier::spline::SplitAt;
use crate::int::math::line::Line;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) struct IntCubeSpline {
    pub(crate) a: IntPoint,
    pub(crate) m: IntPoint,
    pub(crate) b: IntPoint,
}

impl SplitAt for IntCubeSpline {
    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint {
        let l0 = Line::new(self.a, self.m);
        let l1 = Line::new(self.m, self.b);
        let p10 = l0.split_at(step, split_factor);
        let p11 = l1.split_at(step, split_factor);
        Line::new(p10, p11).split_at(step, split_factor)
    }
}