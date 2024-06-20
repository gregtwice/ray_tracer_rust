use std::{f64::NAN, fmt::Debug};

use crate::{object::Shape, ray::Ray, tuple::Tuple, util::EPSILON};

pub struct Intersections(Vec<Intersection>);

#[derive(Clone, Copy)]
pub struct Computations {
    pub i: Intersection,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub inside: bool,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub reflect_v: Tuple,
    /// Refraction calculations
    pub n1: f64,
    /// Refraction calculations
    pub n2: f64,
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    pub time: f64,
    pub object: Shape,
}

impl Intersection {
    pub fn new(t: f64, s: Shape) -> Self {
        Self { time: t, object: s }
    }

    pub fn prepare_computations(&self, r: Ray, xs: &Intersections) -> Computations {
        let mut containers: Vec<Shape> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for x in xs.0.iter() {
            if self == x {
                if containers.is_empty() {
                    n1 = 1.0
                } else {
                    n1 = containers
                        .last()
                        .expect("containers can't be empty")
                        .material
                        .refractive_index;
                }
            }
            if let Some(index) = containers.iter().position(|&s| x.object == s) {
                containers.remove(index);
            } else {
                containers.push(x.object);
            }

            if self == x {
                if containers.is_empty() {
                    n2 = 1.0
                } else {
                    n2 = containers
                        .last()
                        .expect("containers can't be empty")
                        .material
                        .refractive_index;
                }
                break;
            }
        }

        let p = r.position(self.time);
        let mut normal_v = self.object.normal_at(&p);
        let eye_v = -r.direction;
        let inside = if (normal_v.dot(eye_v)) < 0.0 {
            normal_v = -normal_v;
            true
        } else {
            false
        };
        let reflect_v = r.direction.reflect(&normal_v);

        Computations {
            i: *self,
            point: p,
            inside,
            eye_v,
            normal_v,
            over_point: p + normal_v * EPSILON,
            under_point: p - normal_v * EPSILON,
            reflect_v,
            n1,
            n2,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, f64::consts::SQRT_2};

    use crate::{
        intersection::Intersections,
        object::Shape,
        ray::Ray,
        transformations::{scaling, translation},
        tuple::{point, vector},
        util::EPSILON,
    };

    use super::{Intersectable, Intersection};

    #[test]
    fn aggregating_intersections() {
        let s = Shape::sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].time, 1.0);
        assert_eq!(xs.data()[1].time, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere();
        let xs = s.intersects(r);
        assert_eq!(xs.data().len(), 2);
        assert_eq!(xs.data()[0].object, (s));
        assert_eq!(xs.data()[1].object, (s));
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Shape::sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), Some(&i1))
    }

    #[test]
    fn hit_some_intersections_positive_t() {
        let s = Shape::sphere();

        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), Some(&i2))
    }
    #[test]
    fn hit_all_intersections_negative_t() {
        let s = Shape::sphere();

        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), None)
    }

    #[test]
    fn hit_always_lowest_nonnegative_intersection() {
        let s = Shape::sphere();

        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        assert_eq!(xs.hit(), Some(&i4))
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere();
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        assert_eq!(comps.i.object, s);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0))
    }

    #[test]
    fn hit_intersection_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere();

        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_intersection_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere();

        let i = Intersection::new(1.0, s);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::sphere().with_transform(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, s);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let s = Shape::plane();
        let r = Ray::new(
            point(0.0, 1.0, -1.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = Intersection::new(SQRT_2, s);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        assert_eq!(comps.reflect_v, vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let cases = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        let mut a = Shape::glass_sphere().with_transform(scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = Shape::glass_sphere().with_transform(translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = Shape::glass_sphere().with_transform(translation(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let r = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let intersections = vec![
            Intersection::new(2.0, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6.0, a),
        ];
        for (idx, x) in intersections.iter().enumerate() {
            let comps = x.prepare_computations(r, &Intersections::new(intersections.clone()));
            assert_eq!(comps.n1, cases[idx].0);
            assert_eq!(comps.n2, cases[idx].1);
        }
    }

    #[test]
    fn under_point_is_below_the_surface() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::glass_sphere().with_transform(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, s);
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}
