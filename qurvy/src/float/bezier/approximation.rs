use crate::data::list::{EMPTY_REF, LinkList};
use crate::float::bezier::spline::CADSpline;
use crate::float::math::point::Point;

pub trait Approximation {
    fn approximate(&self, min_cos: f64, min_len: f64) -> Vec<Short>;
    fn approximate_points(&self, min_cos: f64, min_len: f64) -> Vec<Point>;
}

impl<Spline: CADSpline> Approximation for Spline {
    #[inline]
    fn approximate(&self, min_cos: f64, min_len: f64) -> Vec<Short> {
        debug_assert!(min_cos <= 1.0);
        Solver::approximate(self, min_cos, min_len)
    }

    #[inline]
    fn approximate_points(&self, min_cos: f64, min_len: f64) -> Vec<Point> {
        let shorts = Solver::approximate(self, min_cos, min_len);
        let mut points: Vec<_> = shorts.iter().map(|s| s.a).collect();
        points.push(shorts.last().unwrap().b);

        points
    }
}

#[derive(Copy, Clone)]
pub struct Short {
    pub step: usize,
    pub split_factor: u32,
    pub dir: Point,
    pub a: Point,
    pub b: Point,
}

struct Solver<'a, Spline> {
    min_cos: f64,
    st_dir: Point,
    ed_dir: Point,
    min_sqr_len: f64,
    spline: &'a Spline,
    segments: LinkList<Short>,
}

impl<'a, Spline: CADSpline> Solver<'a, Spline> {
    #[inline]
    fn approximate(spline: &Spline, min_cos: f64, min_len: f64) -> Vec<Short> {
        let st_dir = spline.start_dir();
        let ed_dir = spline.end_dir();

        let segments = LinkList::new(vec![Short {
            step: 0,
            split_factor: 0,
            dir: (spline.end() - spline.start()).normalized(),
            a: spline.start(),
            b: spline.end(),
        }]);

        Solver {
            min_cos,
            st_dir,
            ed_dir,
            min_sqr_len: min_len * min_len,
            spline,
            segments,
        }
        .process()
    }

    #[inline]
    fn process(&mut self) -> Vec<Short> {
        let mut buffer = Vec::with_capacity(16);
        buffer.push(0);

        let mut to_split = Vec::with_capacity(16);

        while !buffer.is_empty() {
            for index in buffer.iter() {
                if self.split_test(*index) {
                    to_split.push(*index);
                }
            }

            buffer.clear();
            for &index in to_split.iter() {
                self.split(index, &mut buffer);
            }
            to_split.clear();
        }

        let mut shorts = Vec::with_capacity(self.segments.len());
        let mut index = 0;
        while index != EMPTY_REF {
            let node = self.segments.get(index);
            shorts.push(node.item);
            index = node.next
        }

        shorts
    }

    fn split_test(&self, index: u32) -> bool {
        let node = self.segments.get(index);
        let prev = node.prev;
        let next = node.next;
        let dir = node.item.dir;
        let prev_dir = if prev != EMPTY_REF {
            self.segments.get(prev).item.dir
        } else {
            self.st_dir
        };

        let prev_dot_product = dir.dot_product(&prev_dir);
        if prev_dot_product < self.min_cos {
            return true;
        }

        let next_dir = if next != EMPTY_REF {
            self.segments.get(next).item.dir
        } else {
            self.ed_dir
        };

        let next_dot_product = dir.dot_product(&next_dir);
        next_dot_product < self.min_cos
    }

    fn split(&mut self, index: u32, result: &mut Vec<u32>) {
        let short = self.segments.get(index).item;

        let split_factor = short.split_factor + 1;
        let m = self.spline.split_at(short.step + 1, split_factor);
        let ma = m - short.a;
        let bm = short.b - m;

        let s0 = Short {
            step: short.step << 1,
            split_factor,
            dir: ma.normalized(),
            a: short.a,
            b: m,
        };

        let s1 = Short {
            step: (short.step + 1) << 1,
            split_factor,
            dir: bm.normalized(),
            a: m,
            b: short.b,
        };

        let (i0, i1) = self.segments.split_at(index, s0, s1);

        if ma.sqr_length() > self.min_sqr_len {
            result.push(i0)
        }

        if bm.sqr_length() > self.min_sqr_len {
            result.push(i1)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::float::bezier::approximation::Approximation;
    use crate::float::bezier::spline_quad::QuadSpline;
    use crate::float::math::point::Point;

    #[test]
    fn test_00() {
        let spline = QuadSpline {
            a: Point::new(0.0, 0.0),
            am: Point::new(0.0, 50.0),
            bm: Point::new(100.0, 50.0),
            b: Point::new(100.0, 0.0),
        };

        let shorts = spline.approximate(0.8, 4.0);
        assert_eq!(shorts.len(), 8);
    }

    #[test]
    fn test_01() {
        let spline = QuadSpline {
            a: Point::new(0.0, 0.0),
            am: Point::new(0.0, 50.0),
            bm: Point::new(100.0, 50.0),
            b: Point::new(100.0, 0.0),
        };

        let shorts = spline.approximate(0.9, 32.0);
        assert_eq!(shorts.len(), 8);
    }

    #[test]
    fn test_02() {
        let spline = QuadSpline {
            a: Point::new(0.0, 0.0),
            am: Point::new(0.0, 50.0),
            bm: Point::new(100.0, 50.0),
            b: Point::new(100.0, 0.0),
        };

        let shorts = spline.approximate(0.9, 32.0);
        assert_eq!(shorts.len(), 8);
    }
}
