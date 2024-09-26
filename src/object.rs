use std::fmt::Debug;

use slotmap::Key;

use crate::bounds::{Bounded, BoundingBox};
use crate::shapes::prelude::*;

use crate::{
    intersection::{Intersectable, Intersection, Intersections},
    material::Material,
    matrix::{Mat4, MatBase},
    pattern::Pattern,
    ray::Ray,
    tuple::{vector, Tuple},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ShapeType {
    Plane(Plane),
    Cube(Cube),
    Cone(Cone),
    Sphere(Sphere),
    Cylinder(Cylinder),
    TestShape(TestShape),
}
impl Bounded for ShapeType {
    fn bounds(&self) -> BoundingBox {
        match self {
            ShapeType::Plane(p) => p.bounds(),
            ShapeType::Cube(c) => c.bounds(),
            ShapeType::Sphere(s) => s.bounds(),
            ShapeType::Cylinder(c) => c.bounds(),
            ShapeType::TestShape(_) => unimplemented!(),
            ShapeType::Cone(c) => c.bounds(),
        }
    }
}

impl LocalIntersect for ShapeType {
    fn local_intersect(&self, r: Ray) -> Vec<f64> {
        match self {
            ShapeType::Plane(p) => p.local_intersect(r),
            ShapeType::Cube(c) => c.local_intersect(r),
            ShapeType::Sphere(s) => s.local_intersect(r),
            ShapeType::Cylinder(c) => c.local_intersect(r),
            ShapeType::TestShape(t) => t.local_intersect(r),
            ShapeType::Cone(c) => c.local_intersect(r),
        }
    }

    fn local_normal_at(&self, object_point: &Tuple) -> Tuple {
        match self {
            ShapeType::Plane(p) => p.local_normal_at(object_point),
            ShapeType::Cube(c) => c.local_normal_at(object_point),
            ShapeType::Sphere(s) => s.local_normal_at(object_point),
            ShapeType::Cylinder(c) => c.local_normal_at(object_point),
            ShapeType::TestShape(t) => t.local_normal_at(object_point),
            ShapeType::Cone(c) => c.local_normal_at(object_point),
        }
    }
}

pub trait LocalIntersect: Debug + Sync {
    fn local_intersect(&self, r: Ray) -> Vec<f64>;
    fn local_normal_at(&self, object_point: &Tuple) -> Tuple;
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub transform: Mat4,
    pub transform_inverse: Mat4,
    pub material: Material,
    object: ShapeType,
    pub parent: Group,
}

impl Bounded for Shape {
    fn bounds(&self) -> BoundingBox {
        self.object.bounds()
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object
            && self.transform == other.transform
            && self.material == other.material
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            parent: Group::null(),
            transform: Mat4::identity(),
            transform_inverse: Mat4::identity(),
            material: Material::default(),
            object: ShapeType::Sphere(Sphere),
        }
    }
}

impl Shape {
    pub fn sphere() -> Self {
        Self {
            object: ShapeType::Sphere(Sphere),
            ..Default::default()
        }
    }

    pub fn cylinder(min: f64, max: f64, closed: bool) -> Self {
        let c = CylinderBuilder::new()
            .closed(closed)
            .max(max)
            .min(min)
            .build();

        Self {
            object: ShapeType::Cylinder(c),
            ..Default::default()
        }
    }

    pub fn cone(min: f64, max: f64, closed: bool) -> Self {
        let c = ConeBuilder::new().max(max).min(min).closed(closed).build();
        Self {
            object: ShapeType::Cone(c),
            ..Default::default()
        }
    }

    pub fn glass_sphere() -> Self {
        Self {
            object: ShapeType::Sphere(Sphere),
            material: Material::default().refractive_index(1.5).transparency(1.0),
            ..Default::default()
        }
    }

    pub fn cube() -> Self {
        Self {
            object: ShapeType::Cube(Cube),
            ..Default::default()
        }
    }

    pub fn plane() -> Self {
        Self {
            object: ShapeType::Plane(Plane),
            ..Default::default()
        }
    }

    pub fn default_shape() -> Self {
        Self {
            object: ShapeType::TestShape(TestShape),
            ..Default::default()
        }
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self.transform_inverse = transform.inverse();
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
        self.transform_inverse = transform.inverse()
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.material.pattern = Some(pattern)
    }

    pub fn world_to_object(&self, point: Tuple) -> Tuple {
        let parent = self.parent;
        let point = if !parent.is_null() {
            parent.world_to_object(point)
        } else {
            point
        };
        self.transform_inverse * point
    }
    pub fn normal_to_world(&self, normal: Tuple) -> Tuple {
        let inverse = self.transform_inverse.transpose();
        let normal = ((inverse * normal).into_vector()).norm();
        let parent = self.parent;
        if !parent.is_null() {
            parent.normal_to_world(normal)
        } else {
            normal
        }
    }
}

impl Intersectable for Shape {
    fn intersects(&self, r: crate::ray::Ray) -> Intersections {
        let r = r.transform(self.transform_inverse);
        let xs = self.object.local_intersect(r);

        Intersections::new(xs.iter().map(|t| Intersection::new(*t, *self)).collect())
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        let local_point = self.world_to_object(*point);
        let local_normal = self.object.local_normal_at(&local_point);
        self.normal_to_world(local_normal)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TestShape;
impl LocalIntersect for TestShape {
    fn local_intersect(&self, _r: Ray) -> Vec<f64> {
        unimplemented!()
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
