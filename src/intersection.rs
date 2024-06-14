use std::fmt::Debug;

use crate::{material::Material, object::Object, ray::Ray, sphere::Sphere, tuple::Tuple};

pub struct Intersections(Vec<Intersection>);

pub struct Computations {
    i: Intersection,
    point: Tuple,
    inside: bool,
    eye_v: Tuple,
    normal_v: Tuple,
}

impl Intersections {
    pub fn new(i: Vec<Intersection>) -> Intersections {
        Self(i)
    }

    pub fn new_none() -> Self {
        Self(vec![])
    }

    pub fn data(&self) -> &Vec<Intersection> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<Intersection> {
        self.0
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.0
            .iter()
            .filter(|t| t.time > 0.0)
            .min_by(|a, b| a.time.total_cmp(&b.time))
    }
}

pub trait Intersectable: Debug + PartialEq + Sized {
    fn intersects(&self, r: Ray) -> Intersections;

    fn normal_at(&self, point: &Tuple) -> Tuple;

    fn material(&self) -> Material;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    pub time: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: f64, s: &Sphere) -> Self {
        Self {
            time: t,
            object: Object::Sphere(*s),
        }
    }

    pub fn prepare_computations(&self, r: Ray) -> Computations {
        let p = r.position(self.time);
        let mut normal_v = self.object.normal_at(&p);
        let eye_v = -r.direction;
        let inside = if (normal_v.dot(eye_v)) < 0.0 {
            normal_v = -normal_v;
            true
        } else {
            false
        };

        Computations {
            i: *self,
            point: p,
            inside,
            eye_v,
            normal_v,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        intersection::Intersections,
        object::Object,
        ray::Ray,
        sphere::Sphere,
        tuple::{point, vector},
    };

    use super::{Intersectable, Intersection};

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].time, 1.0);
        assert_eq!(xs.data()[1].time, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].object, Object::Sphere(s));
        assert_eq!(xs.data()[1].object, Object::Sphere(s));
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), Some(&i1))
    }

    #[test]
    fn hit_some_intersections_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), Some(&i2))
    }
    #[test]
    fn hit_all_intersections_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), None)
    }

    #[test]
    fn hit_always_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        assert_eq!(xs.hit(), Some(&i4))
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.i.object, Object::Sphere(s));
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0))
    }

    #[test]
    fn hit_intersection_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_intersection_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(1.0, &s);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }
}
