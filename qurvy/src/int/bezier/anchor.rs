use serde::{Deserialize, Serialize};
use crate::convert::grid::Grid;
use crate::convert::to_float::ToFloat;
use crate::float::bezier::anchor::BezierAnchor;
use crate::int::math::offset::IntOffset;
use crate::int::math::point::IntPoint;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IntBezierAnchor {
    pub point: IntPoint,
    pub handle_in: Option<IntOffset>,
    pub handle_out: Option<IntOffset>,
}

impl IntBezierAnchor {

    #[inline]
    pub fn handle_in_point(&self) -> Option<IntPoint> {
        self.handle_in.map(|offset|self.point + offset)
    }

    #[inline]
    pub fn handle_out_point(&self) -> Option<IntPoint> {
        self.handle_out.map(|offset|self.point + offset)
    }
}

impl ToFloat<BezierAnchor> for IntBezierAnchor {
    #[inline]
    fn to_float(&self, grid: &Grid) -> BezierAnchor {
        BezierAnchor {
            point: self.point.to_float(grid),
            handle_in: self.handle_in.map(|handle| handle.to_float(grid)),
            handle_out: self.handle_out.map(|handle| handle.to_float(grid)),
        }
    }
}