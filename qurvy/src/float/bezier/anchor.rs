use serde::{Deserialize, Serialize};
use crate::convert::to_int::ToInt;
use crate::float::math::offset::Offset;
use crate::float::math::point::Point;
use crate::int::bezier::anchor::IntBezierAnchor;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BezierAnchor {
    pub point: Point,
    pub handle_in: Option<Offset>,
    pub handle_out: Option<Offset>,
}

impl BezierAnchor {

    #[inline]
    pub fn handle_in_point(&self) -> Option<Point> {
        self.handle_in.map(|offset|self.point + offset)
    }

    #[inline]
    pub fn handle_out_point(&self) -> Option<Point> {
        self.handle_out.map(|offset|self.point + offset)
    }
}

impl ToInt<IntBezierAnchor> for BezierAnchor {
    #[inline]
    fn to_int(&self, scale: f64) -> IntBezierAnchor {
        IntBezierAnchor {
            point: self.point.to_int(scale),
            handle_in: self.handle_in.map(|handle| handle.to_int(scale)),
            handle_out: self.handle_out.map(|handle| handle.to_int(scale)),
        }
    }
}