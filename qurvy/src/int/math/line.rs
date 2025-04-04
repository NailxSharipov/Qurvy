use crate::int::math::point::IntPoint;

pub(crate) struct IntLine {
    a: IntPoint,
    b: IntPoint
}

impl IntLine {

    #[inline]
    pub(crate) fn new(a: IntPoint, b: IntPoint) -> Self {
        Self { a, b }
    }

    #[inline]
    pub(crate) fn split_at(&self, step: usize, split_factor: u32) -> IntPoint {
        let x = Self::split_one_dimension_at(self.a.x, self.b.x, step, split_factor);
        let y = Self::split_one_dimension_at(self.a.y, self.b.y, step, split_factor);
        IntPoint::new(x, y)
    }

    #[inline]
    fn split_one_dimension_at(a: i64, b: i64, step: usize, split_factor: u32) -> i64 {
        let ab = b.wrapping_sub(a) as i128;
        let step = step as i128;

        let scaled = (ab * step) >> split_factor;

        a.wrapping_add(scaled as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::int::math::line::IntLine;
    use crate::int::math::point::IntPoint;

    #[test]
    fn test_00() {
        let m = IntLine::new(IntPoint::new(0, 0), IntPoint::new(0, 8))
            .split_at(1, 1);
        assert_eq!(m, IntPoint::new(0, 4));
    }

    #[test]
    fn test_01() {
        let line = IntLine::new(IntPoint::new(0, 0), IntPoint::new(0, 8));
        let m0 = line.split_at(0, 2);
        let m1 = line.split_at(1, 2);
        let m2 = line.split_at(2, 2);
        let m3 = line.split_at(3, 2);
        let m4 = line.split_at(4, 2);

        assert_eq!(m0, IntPoint::new(0, 0));
        assert_eq!(m1, IntPoint::new(0, 2));
        assert_eq!(m2, IntPoint::new(0, 4));
        assert_eq!(m3, IntPoint::new(0, 6));
        assert_eq!(m4, IntPoint::new(0, 8));
    }

    #[test]
    fn test_02() {
        let line = IntLine::new(IntPoint::new(0, -4), IntPoint::new(0, 4));
        let m0 = line.split_at(0, 2);
        let m1 = line.split_at(1, 2);
        let m2 = line.split_at(2, 2);
        let m3 = line.split_at(3, 2);
        let m4 = line.split_at(4, 2);

        assert_eq!(m0, IntPoint::new(0, -4));
        assert_eq!(m1, IntPoint::new(0, -2));
        assert_eq!(m2, IntPoint::new(0, 0));
        assert_eq!(m3, IntPoint::new(0, 2));
        assert_eq!(m4, IntPoint::new(0, 4));
    }

    #[test]
    fn test_03() {
        let line = IntLine::new(IntPoint::new(0, -4), IntPoint::new(0, 4));
        let m0 = line.split_at(0, 3);
        let m1 = line.split_at(1, 3);
        let m2 = line.split_at(2, 3);
        let m3 = line.split_at(3, 3);
        let m4 = line.split_at(4, 3);
        let m5 = line.split_at(5, 3);
        let m6 = line.split_at(6, 3);
        let m7 = line.split_at(7, 3);
        let m8 = line.split_at(8, 3);

        assert_eq!(m0, IntPoint::new(0, -4));
        assert_eq!(m1, IntPoint::new(0, -3));
        assert_eq!(m2, IntPoint::new(0, -2));
        assert_eq!(m3, IntPoint::new(0, -1));
        assert_eq!(m4, IntPoint::new(0, 0));
        assert_eq!(m5, IntPoint::new(0, 1));
        assert_eq!(m6, IntPoint::new(0, 2));
        assert_eq!(m7, IntPoint::new(0, 3));
        assert_eq!(m8, IntPoint::new(0, 4));
    }
}