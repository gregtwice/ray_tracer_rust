use std::{
    fmt::Display,
    ops::{Index, IndexMut, Mul},
    usize,
};

use crate::{
    transformations::{rot_x, rot_y, rot_z, scaling, shearing, translation},
    tuple::Tuple,
    util::flt_eq,
};

pub type Mat4 = Matrix<4>;
pub type Mat3 = Matrix<3>;
pub type Mat2 = Matrix<2>;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize> {
    data: [[f64; N]; N],
}

pub trait MatBase: Default + IndexMut<(usize, usize), Output = f64> {
    fn inverse(&self) -> Self;
    fn minor(&self, row: usize, col: usize) -> f64;
    fn cofactor(&self, row: usize, col: usize) -> f64;
    fn det(&self) -> f64;
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self {
            data: [[0.0; N]; N],
        }
    }
}

impl Matrix<2> {
    pub const fn new(data: [f64; 4]) -> Matrix<2> {
        Self {
            data: [[data[0], data[1]], [data[2], data[3]]],
        }
    }

    pub fn det(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }
}

impl Matrix<3> {
    pub const fn new(data: [f64; 9]) -> Matrix<3> {
        Self {
            data: [
                [data[0], data[1], data[2]],
                [data[3], data[4], data[5]],
                [data[6], data[7], data[8]],
            ],
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut v = Vec::with_capacity(4);
        for r in 0..3 {
            for c in 0..3 {
                if c != col && r != row {
                    v.push(self[(r, c)])
                }
            }
        }
        Matrix::<2>::new(v.try_into().unwrap())
    }
}

impl MatBase for Mat3 {
    fn inverse(&self) -> Self {
        let mut m = Mat3::default();
        let det = self.det();
        for r in 0..3 {
            for c in 0..3 {
                // transpose with c<-->r
                m[(c, r)] = self.cofactor(r, c) / det;
            }
        }
        m
    }
    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * (if (row + col) & 1 == 1 { -1.0 } else { 1.0 })
    }
    fn det(&self) -> f64 {
        (0..self.data.len()).fold(0.0, |acc, col| acc + self[(0, col)] * self.cofactor(0, col))
    }
}

impl Matrix<4> {
    pub const fn new(data: [f64; 16]) -> Matrix<4> {
        Self {
            data: [
                [data[0], data[1], data[2], data[3]],
                [data[4], data[5], data[6], data[7]],
                [data[8], data[9], data[10], data[11]],
                [data[12], data[13], data[14], data[15]],
            ],
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            data: [
                [self[(0, 0)], self[(1, 0)], self[(2, 0)], self[(3, 0)]],
                [self[(0, 1)], self[(1, 1)], self[(2, 1)], self[(3, 1)]],
                [self[(0, 2)], self[(1, 2)], self[(2, 2)], self[(3, 2)]],
                [self[(0, 3)], self[(1, 3)], self[(2, 3)], self[(3, 3)]],
            ],
        }
    }
    pub const fn identity() -> Matrix<4> {
        Self::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut v = Vec::with_capacity(9);
        for r in 0..4 {
            for c in 0..4 {
                if c != col && r != row {
                    v.push(self[(r, c)])
                }
            }
        }
        Matrix::<3>::new(v.try_into().unwrap())
    }
    fn as_array(&self) -> [f64; 16] {
        unsafe { std::mem::transmute(self.data) }
    }

    pub fn translation(self, x: f64, y: f64, z: f64) -> Self {
        translation(x, y, z) * self
    }
    pub fn scaling(self, x: f64, y: f64, z: f64) -> Self {
        scaling(x, y, z) * self
    }

    pub fn rot_x(self, angle: f64) -> Self {
        rot_x(angle) * self
    }
    pub fn rot_y(self, angle: f64) -> Self {
        rot_y(angle) * self
    }
    pub fn rot_z(self, angle: f64) -> Self {
        rot_z(angle) * self
    }

    pub fn shearing(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

impl MatBase for Mat4 {
    fn inverse(&self) -> Self {
        let mut m = Self::default();
        let det = self.det();
        for r in 0..4 {
            for c in 0..4 {
                // transpose with c<-->r
                m[(c, r)] = self.cofactor(r, c) / det;
            }
        }
        m
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * (if (row + col) & 1 == 1 { -1.0 } else { 1.0 })
    }

    fn det(&self) -> f64 {
        (0..self.data.len()).fold(0.0, |acc, col| acc + self[(0, col)] * self.cofactor(0, col))
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Matrix<N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        let mut m = Self::default();
        for row in 0..N {
            for col in 0..N {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }
        m
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..N {
            for y in 0..N {
                if !flt_eq(self.data[y][x], other.data[y][x]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut res = [0.0; 4];
        for row in 0..4 {
            res[row] = Tuple::from(self.data[row]) * rhs;
        }
        res.into()
    }
}

impl Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let longest = self.as_array().map(|flt| format!("{:5.5}", flt));
        for i in 0..4 {
            write!(f, "| ")?;
            for j in 0..4 {
                write!(
                    f,
                    "{}{} |",
                    if self.data[i][j] >= 0.0 { " " } else { "" },
                    longest[i * 4 + j]
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        matrix::{Mat4, MatBase},
        tuple::Tuple,
    };

    use super::{Mat3, Matrix};

    #[test]
    fn test_eq() {
        let m = Matrix::<4>::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let m2 = Matrix::<4>::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        assert_eq!(m, m2)
    }

    #[test]
    fn test_neq() {
        let m = Matrix::<4>::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let m2 = Matrix::<4>::new([
            1.0, 2.0, 4.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        assert_ne!(m, m2)
    }
    #[test]
    fn test_mul() {
        let a = Matrix::<4>::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix::<4>::new([
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);
        let c = Matrix::<4>::new([
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        ]);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mul_tuple() {
        let m = Matrix::<4>::new([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(m * t, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_identity() {
        let m = Matrix::<4>::new([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        assert_eq!(m * Matrix::<4>::identity(), m)
    }

    #[test]
    fn test_transpose() {
        assert_eq!(Matrix::<4>::identity(), Matrix::<4>::identity().transpose());

        let a = Matrix::<4>::new([
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);
        let t_a = Matrix::<4>::new([
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        ]);
        assert_eq!(a.transpose(), t_a);
    }

    #[test]
    fn test_discriminant_2x2() {
        let m = Matrix::<2>::new([1.0, 5.0, -3.0, 2.0]);
        assert_eq!(m.det(), 17.0);
    }

    #[test]
    fn test_sub_3_3() {
        let m = Matrix::<3>::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        assert_eq!(m.submatrix(0, 2), Matrix::<2>::new([-3.0, 2.0, 0.0, 6.0]));
    }

    #[test]
    fn test_sub_4_4() {
        let m = Matrix::<4>::new([
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ]);

        assert_eq!(
            m.submatrix(2, 1),
            Matrix::<3>::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
        );
    }

    #[test]
    fn test_minor_3_3() {
        let m = Matrix::<3>::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_det_3x3() {
        let m = Mat3::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.det(), -196.0);
    }

    #[test]
    fn test_det_4x4() {
        let m = Mat4::new([
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.det(), -4071.0);
    }

    #[test]
    fn test_inverse_4x4() {
        let m = Mat4::new([
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
        ]);
        let im = Mat4::new([
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
            0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        ]);
        assert_eq!(m.inverse(), im)
    }

    #[test]
    fn test_inverse_mul_4x4() {
        let a = Mat4::new([
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        ]);
        let b = Mat4::new([
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        ]);

        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }
}
