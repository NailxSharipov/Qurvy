pub trait ToInt<T> {
    fn to_int(&self, scale: f64) -> T;
}