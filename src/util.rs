pub fn flt_eq(a: f64, b: f64) -> bool {
    f64::abs(a - b) < 0.00001
}
