pub trait ToFloat<T> {
    fn to_float(&self, scale: f64) -> T;
}