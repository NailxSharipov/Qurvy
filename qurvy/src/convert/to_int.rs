use crate::convert::grid::Grid;

pub trait ToInt<T> {
    fn to_int(&self, grid: &Grid) -> T;
}