use serde::{Deserialize, Serialize};
use crate::convert::to_int::ToInt;
use crate::float::bezier::anchor::BezierAnchor;
use crate::int::bezier::path::IntBezierPath;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BezierPath {
    pub anchors: Vec<BezierAnchor>,
    pub closed: bool,
}

impl ToInt<IntBezierPath> for BezierPath {
    #[inline]
    fn to_int(&self, scale: f64) -> IntBezierPath {
        IntBezierPath {
            anchors: self.anchors.iter().map(|a|a.to_int(scale)).collect(),
            closed: self.closed,
        }
    }
}