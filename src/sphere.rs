use std::default;

use crate::{
    intersection::{Intersectable, Intersection, Intersections},
    material::Material,
    matrix::{Mat4, MatBase},
    ray::Ray,
    tuple::{point, Tuple},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    transform: Mat4,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersects(&self, r: Ray) -> Intersections {
        let r = r.transform(self.transform.inverse());
        let sphere_to_ray = r.origin - point(0.0, 0.0, 0.0);
        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            Intersections::new_none()
        } else {
            Intersections::new(vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), &self),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), &self),
            ])
        }
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = (self.transform.inverse()) * (*world_point);
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = Mat4::transpose(self.transform.inverse()) * object_normal;
        world_normal.w = 0.0;
        world_normal.norm()
    }

    fn material(&self) -> Material {
        self.material
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Mat4::identity(),
            material: Material::default(),
        }
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }

    pub fn material(&mut self) -> &mut Material {
        &mut self.material
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        intersection::Intersectable,
        matrix::Mat4,
        ray::Ray,
        transformations::{scaling, translation},
        tuple::{point, vector},
    };

    use super::Sphere;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].time, 4.0);
        assert_eq!(xs.data()[1].time, 6.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.data().len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].time, -1.0);
        assert_eq!(xs.data()[1].time, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].time, -6.0);
        assert_eq!(xs.data()[1].time, -4.0);
    }

    #[test]
    fn a_sphere_default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, Mat4::identity());
    }

    #[test]
    fn changing_a_spheres_transform() {
        let mut s = Sphere::new();
        let transform = Mat4::identity().translation(2.0, 3.0, 4.0);
        s.set_transform(transform);
        assert_eq!(s.transform, transform);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersects(r);
        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].time, 3.0);
        assert_eq!(xs.data()[1].time, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersects(r);
        assert_eq!(xs.data().len(), 0);
    }

    #[test]
    fn normal_on_sphere_point_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_point_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_point_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_point_non_axial() {
        let s = Sphere::new();
        let n = s.normal_at(&point(
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            vector(3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0)
        );
    }

    #[test]
    fn normal_on_sphere_is_normalized() {
        let s = Sphere::new();
        let n = s.normal_at(&point(
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.norm());
    }
    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Mat4::identity().translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Mat4::identity().rot_z(PI / 5.0).scaling(1.0, 0.5, 1.0));
        let n = s.normal_at(&point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}
