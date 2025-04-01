use crate::float::math::point::Point;

pub(crate) struct Line {
    a: Point,
    b: Point
}

impl Line {

    #[inline]
    pub(crate) fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    #[inline]
    pub(crate) fn split_at(&self, step: usize, split_factor: u32) -> Point {
        let i_pow = 2f64.powi(-(split_factor as i32));
        let weight = (step as f64) * i_pow;
        self.split_by_weight(weight)
    }

    #[inline]
    pub(crate) fn split_by_weight(&self, weight: f64) -> Point {
        let x = self.a.x + weight * (self.b.x - self.a.x);
        let y = self.a.y + weight * (self.b.y - self.a.y);

        Point::new(x, y)
    }
}