use crate::int::bezier::spline::IntSpline;
use crate::int::bool::core::overlay::ShapeType;

#[derive(Debug, Clone)]
pub(crate) struct ShapeSegment {
    pub(crate) max_split_factor: u32,
    pub(crate) shape_type: ShapeType,
    pub(crate) part: ShapePart
}

#[derive(Debug, Clone)]
pub(crate) enum ShapePart {
    Spline(IntSpline)
}