use crate::convert::grid::Grid;

pub trait ToFloat<T> {
    fn to_float(&self, grid: &Grid) -> T;
}