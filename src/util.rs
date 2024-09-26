pub const EPSILON: f64 = 0.0001;

pub fn flt_eq(a: f64, b: f64) -> bool {
    f64::abs(a - b) < EPSILON
}

pub fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

pub fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}
pub const MAX_REFLECTIONS: usize = 10;
