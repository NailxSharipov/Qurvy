use crate::int::bool::geom::segment::ShapeSegment;

pub(crate) trait SplitSegments {
    fn split_segments(self) -> Vec<ShapeSegment>;
}

impl SplitSegments for Vec<ShapeSegment> {
    fn split_segments(self) -> Vec<ShapeSegment> {
        let mut fragments = Vec::new();
        let split_factor = 3;
        for (i, s) in self.iter().enumerate() {
            let mut sub_segments = s.to_fragments(i, split_factor);
            fragments.append(&mut sub_segments);
        }

        self
    }
}