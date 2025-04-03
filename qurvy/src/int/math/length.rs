use crate::float::bezier::spline_cube::CubeSpline;
use crate::float::bezier::spline_quadratic::QuadraticSpline;
use crate::float::math::length::SplineLength;
use crate::int::bezier::spline::IntSpline;
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::bezier::spline_line::IntLineSpline;
use crate::int::bezier::spline_quadratic::IntQuadraticSpline;

const AVG_LEN_CALCULATION_SPLIT_COUNT_POWER: u32 = 4;

pub(crate) trait IntSplineLength {
    fn avg_length(&self, split_factor: u32) -> u64;

    #[inline]
    fn max_split_factor(&self) -> u32 {
        let len = self.avg_length(AVG_LEN_CALCULATION_SPLIT_COUNT_POWER);
        if len < 4 {
            0
        } else {
            len.ilog2() - 2
        }
    }
}

impl IntSplineLength for IntLineSpline {
    #[inline]
    fn avg_length(&self, _split_factor: u32) -> u64 {
        let ax = self.a.x as f64;
        let ay = self.a.y as f64;
        let bx = self.b.x as f64;
        let by = self.b.y as f64;

        let dx = bx - ax;
        let dy = by - ay;

        (dx * dx + dy * dy).sqrt() as u64
    }
}

impl IntSplineLength for IntCubeSpline {
    #[inline]
    fn avg_length(&self, split_factor: u32) -> u64 {
        let spline: CubeSpline = self.into();
        let len = spline.avg_length(split_factor);
        len.min(u64::MAX as f64) as u64
    }
}

impl IntSplineLength for IntQuadraticSpline {
    #[inline]
    fn avg_length(&self, split_factor: u32) -> u64 {
        let spline: QuadraticSpline = self.into();
        let len = spline.avg_length(split_factor);
        len.min(u64::MAX as f64) as u64
    }
}

impl IntSplineLength for IntSpline {
    #[inline]
    fn avg_length(&self, split_factor: u32) -> u64 {
        match self {
            IntSpline::Line(spline) => spline.avg_length(split_factor),
            IntSpline::Cube(spline) => spline.avg_length(split_factor),
            IntSpline::Tetra(spline) => spline.avg_length(split_factor),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::int::bezier::spline_cube::IntCubeSpline;
    use crate::int::bezier::spline_line::IntLineSpline;
    use crate::int::bezier::spline_quadratic::IntQuadraticSpline;
    use crate::int::math::length::IntSplineLength;
    use crate::int::math::point::IntPoint;

    #[test]
    fn test_00() {
        let spline = IntLineSpline {
            a: IntPoint::new(0, 0),
            b: IntPoint::new(100, 100),
        };

        assert_eq!(spline.avg_length(4) as usize, 141);
        assert_eq!(spline.max_split_factor(), 5);
    }

    #[test]
    fn test_01() {
        let spline = IntCubeSpline {
            a: IntPoint::new(0, 0),
            m: IntPoint::new(0, 100),
            b: IntPoint::new(100, 100),
        };

        assert_eq!(spline.avg_length(4) as usize, 162);
        assert_eq!(spline.max_split_factor(), 5);
    }

    #[test]
    fn test_02() {
        let spline = IntQuadraticSpline {
            a: IntPoint::new(0, 0),
            am: IntPoint::new(0, 50),
            bm: IntPoint::new(50, 100),
            b: IntPoint::new(100, 100),
        };

        assert_eq!(spline.avg_length(4) as usize, 154);
        assert_eq!(spline.max_split_factor(), 5);
    }
}