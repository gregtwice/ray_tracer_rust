pub const EPSILON: f64 = 0.00001;

pub fn flt_eq(a: f64, b: f64) -> bool {
    f64::abs(a - b) < EPSILON
}

pub const MAX_REFLECTIONS: usize = 10;
