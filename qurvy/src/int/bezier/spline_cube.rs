use crate::int::bezier::spline::IntCADSpline;
use crate::int::math::line::IntLine;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone)]
pub(crate) struct IntCubeSpline {
    pub(crate) a: IntPoint,
    pub(crate) m: IntPoint,
    pub(crate) b: IntPoint,
}

impl IntCADSpline for IntCubeSpline {
    #[inline]
    fn start(&self) -> IntPoint {
        self.a
    }
    #[inline]
    fn start_dir(&self) -> IntPoint {
        (self.m - self.a).normalized_10bit()
    }
    #[inline]
    fn end_dir(&self) -> IntPoint {
        (self.b - self.m).normalized_10bit()
    }
    #[inline]
    fn end(&self) -> IntPoint {
        self.b
    }

    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint {
        let l0 = IntLine::new(self.a, self.m);
        let l1 = IntLine::new(self.m, self.b);
        let p10 = l0.split_at(step, split_factor);
        let p11 = l1.split_at(step, split_factor);
        IntLine::new(p10, p11).split_at(step, split_factor)
    }
}
