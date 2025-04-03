use crate::int::bezier::spline::SplinePointsIter;
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::bool::geom::fragment::Fragment;

impl IntCubeSpline {
    // #[inline]
    // fn to_fragments(&self, id: usize) -> Vec<Fragment> {
    //     let split_factor = 2;
    //     let mut iter = self.points_iter(true, true, split_factor);
    //     let mut result = Vec::with_capacity(1 << split_factor);
    //     let mut a = if let Some(first) = iter.next() {
    //         first
    //     } else {
    //         return result;
    //     };
    //
    //     for (step, b) in iter.enumerate() {
    //         result.push(crate::int::bool::geom::fragment::Fragment::with_ab(id, step, split_factor, a, b));
    //         a = b;
    //     }
    //
    //     result
    // }
}