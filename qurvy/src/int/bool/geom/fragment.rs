use crate::int::math::point::IntPoint;
use crate::int::math::rect::IntRect;

pub(crate) struct Fragment {
    pub(crate) step: usize,
    pub(crate) split_factor: u32,
    pub(crate) segment_id: usize,
    pub(crate) boundary: IntRect,
}

impl Fragment {

    #[inline]
    pub(crate) fn new(segment_id: usize, step: usize, split_factor: u32, boundary: IntRect) -> Self {
        Self {
            step,
            split_factor,
            segment_id,
            boundary,
        }
    }

    #[inline]
    pub(crate) fn with_ab(
        segment_id: usize,
        step: usize,
        split_factor: u32,
        a: IntPoint,
        b: IntPoint,
    ) -> Self {
        let ab = b - a;
        let n = ab.half_normal();
        let an = a + n;
        let bn = b + n;
        let mut boundary = IntRect::empty();
        boundary.add_point(&a);
        boundary.add_point(&an);
        boundary.add_point(&b);
        boundary.add_point(&bn);

        Fragment {
            step,
            split_factor,
            segment_id,
            boundary,
        }
    }
}

impl IntPoint {
    #[inline]
    fn half_normal(&self) -> IntPoint {
        let x = -self.y / 2;
        let y = self.x / 2;
        IntPoint::new(x, y)
    }
}