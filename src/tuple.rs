use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use crate::util::flt_eq;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<[f64; 4]> for Tuple {
    fn from(value: [f64; 4]) -> Self {
        Tuple::new(value[0], value[1], value[2], value[3])
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn mag(&self) -> f64 {
        assert!(self.w == 0.0);
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn norm(&self) -> Self {
        assert!(self.w == 0.0);
        *self / self.mag()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        assert!(self.w == 0.0);
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        assert_eq!(self.w, 0.0);
        assert_eq!(normal.w, 0.0);
        *self - *normal * 2.0 * self.dot(*normal)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        assert!(self.w == 0.0);
        assert!(rhs.w == 0.0);
        vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        flt_eq(self.x, other.x)
            && flt_eq(self.y, other.y)
            && flt_eq(self.z, other.z)
            && flt_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}
impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}
impl Mul<Tuple> for Tuple {
    type Output = f64;

    fn mul(self, rhs: Tuple) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl AddAssign for Tuple {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    #[test]
    fn reflect_vector_45() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(&n), vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_vector_slanted() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0);
        assert_eq!(v.reflect(&n), vector(1.0, 0.0, 0.0));
    }
}
