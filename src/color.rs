use std::ops::{Add, Mul, Sub};

use crate::tuple::{vector, Tuple};
#[derive(Debug, Clone, Copy, Default)]
pub struct Color(Tuple);

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(vector(r, g, b))
    }

    #[inline]
    pub fn r(&self) -> f64 {
        self.0.x
    }

    #[inline]
    pub fn g(&self) -> f64 {
        self.0.y
    }

    #[inline]
    pub fn b(&self) -> f64 {
        self.0.z
    }

    pub const fn black() -> Color {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn white() -> Color {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.0.x * rhs.0.x, self.0.y * rhs.0.y, self.0.z * rhs.0.z)
    }
}

// hadamard_product
