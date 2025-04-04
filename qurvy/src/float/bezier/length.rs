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

#[cfg(test)]
mod tests {
    use crate::float::bezier::spline_cube::CubeSpline;
    use crate::float::bezier::spline_line::LineSpline;
    use crate::float::bezier::spline_quad::QuadSpline;
    use crate::float::math::length::SplineLength;
    use crate::float::math::point::Point;

    #[test]
    fn test_00() {
        let spline = LineSpline {
            a: Point::new(0.0, 0.0),
            b: Point::new(100.0, 100.0),
        };

        assert_eq!(spline.avg_length(800.0, 3.0) as usize, 141);
    }

    #[test]
    fn test_01() {
        let spline = CubeSpline {
            a: Point::new(0.0, 0.0),
            m: Point::new(0.0, 100.0),
            b: Point::new(100.0, 100.0),
        };

        assert_eq!(spline.avg_length(800.0, 3.0) as usize, 162);
    }

    #[test]
    fn test_02() {
        let spline = QuadSpline {
            a: Point::new(0.0, 0.0),
            am: Point::new(0.0, 50.0),
            bm: Point::new(50.0, 100.0),
            b: Point::new(100.0, 100.0),
        };

        assert_eq!(spline.avg_length(800.0, 3.0) as usize, 154);
    }
}