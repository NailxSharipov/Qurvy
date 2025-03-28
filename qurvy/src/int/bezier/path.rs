use serde::{Deserialize, Serialize};
use crate::convert::to_float::ToFloat;
use crate::float::bezier::path::BezierPath;
use crate::int::bezier::anchor::IntBezierAnchor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntBezierPath {
    pub anchors: Vec<IntBezierAnchor>,
    pub closed: bool,
}

impl ToFloat<BezierPath> for IntBezierPath {
    #[inline]
    fn to_float(&self, scale: f64) -> BezierPath {
        BezierPath {
            anchors: self.anchors.iter().map(|a|a.to_float(scale)).collect(),
            closed: self.closed,
        }
    }
}