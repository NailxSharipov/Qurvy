use std::collections::{HashMap, LinkedList};
use crate::float::math::point::Point;
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::math::point::IntPoint;

pub trait Approximation {
    fn approximate(&self, max_angle: f64) -> Vec<IntPoint>;
}

struct SplitSegment {
    step: usize,
    direction: Point,
    point: IntPoint
}

impl Approximation for IntCubeSpline {
    fn approximate(&self, max_angle: f64) -> Vec<IntPoint> {
        // let avg_capacity = 16;
        // let min_cos = max_angle.cos();
        // let mut points = Vec::with_capacity(avg_capacity);
        //
        // let sv = self.m - self.a;
        // let ev = self.b - self.m;
        //
        // let mut data = LinkedList::from([
        //     SplitSegment {
        //         step: 0,
        //         direction: direction(self.a, self.m),
        //         point: self.a
        //     },
        //     SplitSegment {
        //         step: 1,
        //         direction: direction(self.m, self.b),
        //         point: self.b
        //     }
        // ]);
        //
        // let mut split_factor = 0;
        //
        // let mut split_indices = HashMap::new();
        //
        // while !split_indices.is_empty() {
        //
        //
        //
        //     split_indices.clear();
        //
        //     for n in data.iter_mut() {
        //
        //     }
        //
        //     split_factor += 1;
        // }
        //
        // points.iter().map(|sp|sp.point).collect()

        vec![]
    }
}

#[inline]
fn direction(a: IntPoint, b: IntPoint) -> Point {
    let pa: Point = a.into();
    let pb: Point = b.into();
    (pb - pa).normalize()
}
//
// #[inline]
// fn is_split_required(a: IntPoint, b: IntPoint, c: IntPoint) -> bool {
//
//
// }