use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use iced::Vector;
use qurvy::int::math::point::IntPoint as QPoint;
use crate::compat::convert::Convert;

// to FloatPoint
impl Convert<FloatPoint<f32>> for QPoint {
    fn convert(&self) -> FloatPoint<f32> {
        FloatPoint::new(self.x as f32, self.y as f32)
    }
}

impl Convert<FloatPoint<f32>> for IntPoint {
    fn convert(&self) -> FloatPoint<f32> {
        FloatPoint::new(self.x as f32, self.y as f32)
    }
}

impl Convert<FloatPoint<f32>> for Vector<f32> {
    fn convert(&self) -> FloatPoint<f32> {
        FloatPoint::new(self.x, self.y)
    }
}

// to Vector
impl Convert<Vector<f32>> for QPoint {
    fn convert(&self) -> Vector<f32> {
        Vector::new(self.x as f32, self.y as f32)
    }
}

impl Convert<Vector<f32>> for IntPoint {
    fn convert(&self) -> Vector<f32> {
        Vector::new(self.x as f32, self.y as f32)
    }
}

impl Convert<Vector<f32>> for FloatPoint<f32> {
    fn convert(&self) -> Vector<f32> {
        Vector::new(self.x, self.y)
    }
}

impl Convert<IntPoint> for QPoint {
    fn convert(&self) -> IntPoint {
        IntPoint::new(self.x as i32, self.y as i32)
    }
}

impl Convert<QPoint> for IntPoint {
    fn convert(&self) -> QPoint {
        QPoint::new(self.x as i64, self.y as i64)
    }
}