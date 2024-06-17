use std::fmt::Debug;

use crate::{
    intersection::{Intersectable, Intersection, Intersections},
    material::Material,
    matrix::{Mat4, MatBase},
    pattern::Pattern,
    plane::Plane,
    ray::Ray,
    sphere::Sphere,
    tuple::{vector, Tuple},
};

pub trait LocalIntersect: Debug + PartialEq {
    fn local_intersect(&self, r: Ray) -> Vec<f64>;
    fn local_normal_at(&self, object_point: &Tuple) -> Tuple;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    No(TestShape),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub transform: Mat4,
    pub material: Material,
    object: Object,
}

impl Shape {
    pub fn sphere() -> Self {
        Self {
            transform: Mat4::identity(),
            material: Material::default(),
            object: Object::Sphere(Sphere),
        }
    }

    pub fn plane() -> Self {
        Self {
            transform: Mat4::identity(),
            material: Material::default(),
            object: Object::Plane(Plane),
        }
    }

    pub fn default_shape() -> Self {
        Self {
            transform: Mat4::identity(),
            material: Material::default(),
            object: Object::No(TestShape),
        }
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.material.pattern = Some(pattern);
        self
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.material.pattern = Some(pattern)
    }
}

impl Intersectable for Shape {
    fn intersects(&self, r: crate::ray::Ray) -> Intersections {
        let r = r.transform(self.transform.inverse());
        let xs = match self.object {
            Object::Sphere(s) => s.local_intersect(r),
            Object::No(_) => unimplemented!(),
            Object::Plane(p) => p.local_intersect(r),
        };

        Intersections::new(xs.iter().map(|t| Intersection::new(*t, *self)).collect())
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        let local_point = (self.transform.inverse()) * (*point);
        let local_normal = match self.object {
            Object::Sphere(s) => s.local_normal_at(&local_point),
            Object::No(ts) => ts.local_normal_at(&local_point),
            Object::Plane(p) => p.local_normal_at(&local_point),
        };
        let mut world_normal = Mat4::transpose(self.transform.inverse()) * local_normal;
        world_normal.w = 0.0;
        world_normal.norm()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TestShape;
impl LocalIntersect for TestShape {
    fn local_intersect(&self, _r: Ray) -> Vec<f64> {
        todo!()
    }

    fn local_normal_at(&self, object_point: &Tuple) -> Tuple {
        vector(object_point.x, object_point.y, object_point.z)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        transformations::{rot_z, translation},
        tuple::point,
    };

    use super::*;

    #[test]
    fn default_transformation() {
        let s = Shape::default_shape();
        assert_eq!(s.transform, Mat4::identity())
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = Shape::default_shape();
        s.set_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0))
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = Shape::default_shape();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
        assert_eq!(n, vector(0.0, 0.70711, -0.70711))
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = Shape::default_shape().with_transform(rot_z(PI / 5.0).scaling(1.0, 0.5, 1.0));
        let n = s.normal_at(&point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
        assert_eq!(n, vector(0.0, 0.97014, -0.24254))
    }
}
