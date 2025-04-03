use crate::float::bezier::spline::SplinePointsIter;
use crate::float::bezier::spline_cube::CubeSpline;
use crate::float::bezier::spline_line::LineSpline;
use crate::float::bezier::spline_quadratic::QuadraticSpline;

pub(crate) trait SplineLength {
    fn avg_length(&self, split_factor: u32) -> f64;
}

impl SplineLength for LineSpline {

    #[inline]
    fn avg_length(&self, _split_factor: u32) -> f64 {
        self.a.distance(self.b)
    }
}

impl SplineLength for CubeSpline {
    fn avg_length(&self, split_factor: u32) -> f64 {
        let mut iter = self.points_iter(true, true, split_factor);
        let mut a = if let Some(first) = iter.next() {
            first
        } else {
          return 0.0;
        };

        let mut s = 0.0;
        for b in iter {
            s += a.distance(b);
            a = b;
        }

        s
    }
}

impl SplineLength for QuadraticSpline {
    fn avg_length(&self, split_factor: u32) -> f64 {
        let mut iter = self.points_iter(true, true, split_factor);
        let mut a = if let Some(first) = iter.next() {
            first
        } else {
            return 0.0;
        };

        let mut s = 0.0;
        for b in iter {
            s += a.distance(b);
            a = b;
        }

        s
    }
}