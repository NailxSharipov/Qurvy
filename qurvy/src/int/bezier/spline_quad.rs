use crate::int::bezier::spline::IntCADSpline;
use crate::int::math::line::IntLine;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) struct IntQuadSpline {
    pub(crate) a: IntPoint,
    pub(crate) am: IntPoint,
    pub(crate) bm: IntPoint,
    pub(crate) b: IntPoint,
}
impl IntCADSpline for IntQuadSpline {
    #[inline]
    fn start(&self) -> IntPoint {
        self.a
    }
    #[inline]
    fn start_dir(&self) -> IntPoint {
        (self.am - self.a).normalized_10bit()
    }
    #[inline]
    fn end_dir(&self) -> IntPoint {
        (self.b - self.bm).normalized_10bit()
    }
    #[inline]
    fn end(&self) -> IntPoint {
        self.b
    }

    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint {
        let l0 = IntLine::new(self.a, self.am);
        let l1 = IntLine::new(self.am, self.bm);
        let l2 = IntLine::new(self.bm, self.b);

        let p0 = l0.split_at(step, split_factor);
        let p1 = l1.split_at(step, split_factor);
        let p2 = l2.split_at(step, split_factor);

        let p10 = IntLine::new(p0, p1).split_at(step, split_factor);
        let p11 = IntLine::new(p1, p2).split_at(step, split_factor);

        IntLine::new(p10, p11).split_at(step, split_factor)
    }
}
