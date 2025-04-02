use crate::int::bezier::spline::{IntSpline, SplinePointsIter};
use crate::int::bezier::spline_cube::IntCubeSpline;
use crate::int::bezier::spline_line::IntLineSpline;
use crate::int::bezier::spline_tetra::IntTetraSpline;
use crate::int::bool::core::overlay::ShapeType;
use crate::int::bool::geom::fragment::Fragment;

#[derive(Debug, Clone)]
pub(crate) struct ShapeSegment {
    pub(crate) max_split_factor: u32,
    pub(crate) shape_type: ShapeType,
    pub(crate) part: ShapePart,
}

#[derive(Debug, Clone)]
pub(crate) enum ShapePart {
    Spline(IntSpline),
}

impl ShapeSegment {

    #[inline]
    pub(crate) fn to_fragments(&self, id: usize, split_factor: u32) -> Vec<Fragment> {
        let factor = split_factor.min(self.max_split_factor);
        match &self.part {
            ShapePart::Spline(spline) => spline.to_fragments(id, factor),
        }
    }
}

impl IntSpline {
    #[inline]
    fn to_fragments(&self, id: usize, split_factor: u32) -> Vec<Fragment> {
        match self {
            IntSpline::Line(spline) => vec![spline.to_fragment(id)],
            IntSpline::Cube(spline) => spline.to_fragments(id, split_factor),
            IntSpline::Tetra(spline) => spline.to_fragments(id, split_factor),
        }
    }
}

impl IntLineSpline {
    #[inline]
    fn to_fragment(&self, id: usize) -> Fragment {
        Fragment {
            step: 0,
            split_factor: 0,
            segment_id: id,
            boundary: self.boundary(),
        }
    }
}

impl IntCubeSpline {
    #[inline]
    fn to_fragments(&self, id: usize, split_factor: u32) -> Vec<Fragment> {
        let mut iter = self.points_iter(true, true, split_factor);
        let mut result = Vec::with_capacity(1 << split_factor);
        let mut a = if let Some(first) = iter.next() {
            first
        } else {
            return result;
        };

        for (step, b) in iter.enumerate() {
            result.push(Fragment::with_ab(id, step, split_factor, a, b));
            a = b;
        }

        result
    }
}

impl IntTetraSpline {
    #[inline]
    fn to_fragments(&self, id: usize, split_factor: u32) -> Vec<Fragment> {
        let mut iter = self.points_iter(true, true, split_factor);
        let mut result = Vec::with_capacity(1 << split_factor);
        let mut a = if let Some(first) = iter.next() {
            first
        } else {
            return result;
        };

        for (step, b) in iter.enumerate() {
            result.push(Fragment::with_ab(id, step, split_factor, a, b));
            a = b;
        }

        result
    }
}
