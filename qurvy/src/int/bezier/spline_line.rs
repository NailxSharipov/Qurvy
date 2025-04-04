use crate::int::bezier::spline::IntCADSpline;
use crate::int::math::line::IntLine;
use crate::int::math::point::IntPoint;
use crate::int::math::rect::IntRect;

#[derive(Debug, Clone)]
pub(crate) struct IntLineSpline {
    pub(crate) a: IntPoint,
    pub(crate) b: IntPoint,
}

impl IntCADSpline for IntLineSpline {
    #[inline]
    fn start(&self) -> IntPoint {
        self.a
    }
    #[inline]
    fn start_dir(&self) -> IntPoint {
        (self.b - self.a).normalized_10bit()
    }

    #[inline]
    fn end_dir(&self) -> IntPoint {
        (self.b - self.a).normalized_10bit()
    }
    #[inline]
    fn end(&self) -> IntPoint {
        self.b
    }

    #[inline]
    fn split_at(&self, step: usize, split_factor: u32) -> IntPoint {
        IntLine::new(self.a, self.b).split_at(step, split_factor)
    }
}

impl IntLineSpline {
    #[inline]
    pub(crate) fn boundary(&self) -> IntRect {
        let (min_x, max_x) = if self.a.x < self.b.x {
            (self.a.x, self.b.x)
        } else {
            (self.b.x, self.a.x)
        };
        let (min_y, max_y) = if self.a.y < self.b.y {
            (self.a.y, self.b.y)
        } else {
            (self.b.y, self.a.y)
        };
        IntRect {
            min: IntPoint { x: min_x, y: min_y },
            max: IntPoint { x: max_x, y: max_y },
        }
    }
}
