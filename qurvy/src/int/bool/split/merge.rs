use crate::convert::grid::Grid;
use crate::int::bool::geom::segment::ShapeSegment;

pub(super) trait Merge {
    fn merge(self, grid: &Grid) -> Vec<ShapeSegment>;
}

impl Merge for Vec<ShapeSegment> {

    fn merge(self, grid: &Grid) -> Vec<ShapeSegment> {
        let mut segments = self;
        //
        // for s in segments.iter() {
        //     s.to_fragments()
        // }
        //
        //

        segments
    }
}