use crate::float::bezier::approximation::Approximation;
use crate::float::bezier::spline::CADSpline;
use crate::float::math::point::Point;

pub(crate) trait SplineLength {
    fn avg_length(&self, min_cos: f64, min_len: f64) -> f64;
}

impl<Spline: CADSpline> SplineLength for Spline {
    fn avg_length(&self, min_cos: f64, min_len: f64) -> f64 {
        let points = self.approximate_points(min_cos, min_len);
        let mut len = 0f64;
        for w in points.windows(2) {
            let a: Point = w[0].into();
            let b: Point = w[1].into();
            len += (a - b).length()
        }

        len
    }
}