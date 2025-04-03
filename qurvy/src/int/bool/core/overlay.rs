use crate::int::bezier::path::IntBezierPath;
use crate::int::bool::geom::segment::{ShapePart, ShapeSegment};
use crate::int::math::length::IntSplineLength;

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Subject,
    Clip,
}

#[derive(Clone)]
pub struct Overlay {
    pub(crate) sections: Vec<ShapeSegment>,
}

impl Overlay {

    #[inline]
    pub fn new() -> Self {
        Self { sections: Vec::new() }
    }

    #[inline]
    pub fn add_bezier_path(&mut self, path: &IntBezierPath, shape_type: ShapeType) {
        // for spline in path.splines() {
        //     let segment = ShapeSegment {
        //         max_split_factor: spline.max_split_factor(),
        //         part: ShapePart::Spline(spline),
        //         shape_type
        //     };
        //     self.sections.push(segment);
        // }
    }
}