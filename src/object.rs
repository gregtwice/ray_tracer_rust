use crate::{intersection::Intersectable, sphere::Sphere, tuple::Tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
}

impl Intersectable for Object {
    fn intersects(&self, r: crate::ray::Ray) -> crate::intersection::Intersections {
        match self {
            Object::Sphere(s) => s.intersects(r),
        }
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        match self {
            Object::Sphere(s) => s.normal_at(point),
        }
    }

    fn material(&self) -> crate::material::Material {
        match self {
            Object::Sphere(s) => s.material(),
        }
    }
}
