
pub struct Grid {
    size_power: u32, // power of 2, default is 4 (2^4 = 16)
    scale_to_int: f64,
    scale_to_float: f64,
    remainder_mask: i64,
}

impl Grid {

    pub fn new(scale_power: i32, size_power: u32) -> Self {
        let e = scale_power + size_power as i32;
        let scale_to_int = 2f64.powi(e);
        let scale_to_float = 2f64.powi(-e);

        let remainder_mask = (1 << size_power) - 1;

        Self { size_power, scale_to_int, scale_to_float, remainder_mask }
    }

    pub fn debug() -> Self {
        Self::new(10,3)
    }

    #[inline(always)]
    fn snap_to_grid_value(&self, a: i64) -> i64 {
        let p = self.size_power;
        let s = ((a << 1) >> p) & 1;
        let c = a >> p;
        (c + s) << p
    }

    #[inline(always)]
    pub fn int_to_float(&self, a: i64) -> f64 {
        a as f64 * self.scale_to_float
    }

    #[inline(always)]
    pub fn float_to_int(&self, a: f64) -> i64 {
        self.snap_to_grid_value((a * self.scale_to_int) as i64)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(20,4)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::grid::Grid;

    #[test]
    fn test_0() {
        let grid = Grid::new(0, 4);
        assert_eq!(grid.snap_to_grid_value(7), 0);
    }

    #[test]
    fn test_1() {
        let grid = Grid::new(0, 4);
        assert_eq!(grid.snap_to_grid_value(8), 16);
    }

    #[test]
    fn test_2() {
        let grid = Grid::new(0, 4);
        assert_eq!(grid.snap_to_grid_value(-7), 0);
    }

    #[test]
    fn test_3() {
        let grid = Grid::new(0, 4);
        assert_eq!(grid.snap_to_grid_value(-9), -16);
    }
}