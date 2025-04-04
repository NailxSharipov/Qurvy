use crate::int::bool::geom::segment::ShapeSegment;

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

    // #[inline]
    // pub fn add_bezier_path(&mut self, path: &IntBezierPath, shape_type: ShapeType) {
    //
    // }
}