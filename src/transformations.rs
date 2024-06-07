use crate::matrix::Mat4;

pub fn translation(x: f64, y: f64, z: f64) -> Mat4 {
    let mut m = Mat4::identity();
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;
    m
}

pub fn scaling(x: f64, y: f64, z: f64) -> Mat4 {
    let mut m = Mat4::identity();
    m[(0, 0)] = x;
    m[(1, 1)] = y;
    m[(2, 2)] = z;
    m
}

pub fn rot_x(angle: f64) -> Mat4 {
    let mut m = Mat4::identity();
    m[(1, 1)] = angle.cos();
    m[(1, 2)] = -angle.sin();
    m[(2, 1)] = angle.sin();
    m[(2, 2)] = angle.cos();
    m
}

pub fn rot_y(angle: f64) -> Mat4 {
    let mut m = Mat4::identity();
    m[(0, 0)] = angle.cos();
    m[(0, 2)] = angle.sin();
    m[(2, 0)] = -angle.sin();
    m[(2, 2)] = angle.cos();
    m
}
pub fn rot_z(angle: f64) -> Mat4 {
    let mut m = Mat4::identity();
    m[(0, 0)] = angle.cos();
    m[(0, 1)] = -angle.sin();
    m[(1, 0)] = angle.sin();
    m[(1, 1)] = angle.cos();
    m
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Mat4 {
    let mut m = Mat4::identity();
    m[(0, 1)] = xy;
    m[(0, 2)] = xz;

    m[(1, 0)] = yx;
    m[(1, 2)] = yz;

    m[(2, 0)] = zx;
    m[(2, 1)] = zy;

    m
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{matrix::MatBase, tuple::*};

    use super::*;

    #[test]
    fn test_translation() {
        let t = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(t * p, point(2.0, 1.0, 7.0));
        assert_eq!(t.inverse() * p, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(t * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let result = transform * p;
        assert_eq!(result, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let t = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(t * p, point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rot_x(PI / 4.0);
        let full_quarter = rot_x(PI / 2.0);
        assert_eq!(half_quarter * p, point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * p, point(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rot_x(PI / 4.0);
        let inv = half_quarter.inverse();
        assert_eq!(inv * p, point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0))
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rot_y(PI / 4.0);
        let full_quarter = rot_y(PI / 2.0);
        assert_eq!(half_quarter * p, point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rot_z(PI / 4.0);
        let full_quarter = rot_z(PI / 2.0);
        assert_eq!(half_quarter * p, point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
        assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1, 0, 0.0, 0, 0, 0);

        let p = point(2, 3, 4);

        assert_eq!(transform * p, point(5, 3, 4));
    }
}
