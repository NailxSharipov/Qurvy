use crate::int::math::rect::IntRect;

pub(crate) struct Fragment {
    step: usize,
    split_factor: usize,
    segment_index: usize,
    boundary: IntRect,
}

trait Fragmentation {
    fn split_by_factor(split_factor: usize) -> Vec<Fragment>;
}