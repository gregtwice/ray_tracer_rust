use core::f64;

use rayon::vec;

use crate::{
    object::LocalIntersect,
    ray::Ray,
    tuple::vector,
    util::{flt_eq, EPSILON},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
    closed: bool,
}

impl Cylinder {
    fn check_cap(r: Ray, t: f64) -> bool {
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;

        return (x * x + z * z) <= 1.;
    }

    fn intersect_caps(&self, r: Ray, xs: &mut Vec<f64>) {
        if !self.closed || flt_eq(r.direction.y, EPSILON) {
            return;
        }
        let t = (self.min - r.origin.y) / r.direction.y;
        if Self::check_cap(r, t) {
            xs.push(t);
        }

        let t = (self.max - r.origin.y) / r.direction.y;
        if Self::check_cap(r, t) {
            xs.push(t);
        }
    }
}

pub struct CylinderBuilder {
    cylinder: Cylinder,
}
impl CylinderBuilder {
    pub fn new() -> Self {
        Self {
            cylinder: Cylinder::default(),
        }
    }

    pub fn min(mut self, min: f64) -> Self {
        self.cylinder.min = min;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.cylinder.max = max;
        self
    }
    pub fn closed(mut self, closed: bool) -> Self {
        self.cylinder.closed = closed;
        self
    }

    pub fn build(self) -> Cylinder {
        self.cylinder
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }
}

impl LocalIntersect for Cylinder {
    fn local_intersect(&self, r: crate::ray::Ray) -> Vec<f64> {
        let mut xs = Vec::new();
        let a = r.direction.x.powi(2) + r.direction.z.powi(2);
        if a <= EPSILON {
            self.intersect_caps(r, &mut xs);
            return xs;
        }

        let b = 2. * r.origin.x * r.direction.x + 2. * r.origin.z * r.direction.z;
        let c = r.origin.x.powi(2) + r.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4. * a * c;
        if disc < 0. {
            return xs;
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1)
        }
        let y0 = r.origin.y + t0 * r.direction.y;
        if self.min < y0 && y0 < self.max {
            xs.push(t0)
        }
        let y1 = r.origin.y + t1 * r.direction.y;
        if self.min < y1 && y1 < self.max {
            xs.push(t1)
        }
        self.intersect_caps(r, &mut xs);
        return xs;
    }

    fn local_normal_at(&self, object_point: &crate::tuple::Tuple) -> crate::tuple::Tuple {
        let dist = object_point.x.powi(2) + object_point.z.powi(2);

        if dist < 1. && object_point.y >= (self.max - EPSILON) {
            vector(0.0, 1., 0.0)
        } else if dist < 1. && object_point.y <= (self.min + EPSILON) {
            vector(0.0, -1.0, 0.0)
        } else {
            vector(object_point.x, 0.0, object_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        tuple::{point, vector},
        util::flt_eq,
    };

    use super::*;

    #[test]
    fn ray_misses_cylinder() {
        let c = Cylinder::default();
        assert_eq!(
            c.local_intersect(Ray::new(point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)))
                .len(),
            0
        );
        assert_eq!(
            c.local_intersect(Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)))
                .len(),
            0
        );
        assert_eq!(
            c.local_intersect(Ray::new(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)))
                .len(),
            0
        );
    }

    #[test]
    fn ray_intersects_cylinder() {
        let c = Cylinder::default();
        let xs = c.local_intersect(Ray::new(point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);

        let xs = c.local_intersect(Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);

        let xs = c.local_intersect(Ray::new(
            point(0.5, 0.0, -5.0),
            vector(0.1, 1.0, 1.0).norm(),
        ));
        assert_eq!(xs.len(), 2);
        assert!(flt_eq(xs[0], 6.80798));
        assert!(flt_eq(xs[1], 7.08872));
    }

    #[test]
    fn cylinder_local_normal_at() {
        let cases = vec![
            (point(1., 0., 0.), vector(1., 0., 0.)),
            (point(0., 5., -1.), vector(0., 0., -1.)),
            (point(0., -2., 1.), vector(0., 0., 1.)),
            (point(-1., 1., 0.), vector(-1., 0., 0.)),
        ];
        for (point, normal) in cases.into_iter() {
            let c = Cylinder::default();
            assert!(c.local_normal_at(&point) == normal);
        }
    }

    #[test]
    fn cylinder_intersects_with_min_max() {
        let cases = vec![
            (point(0., 1.5, 0.), vector(0.1, 1., 0.), 0),
            (point(0., 3., -5.), vector(0., 0., 1.), 0),
            (point(0., 0., -5.), vector(0., 0., 1.), 0),
            (point(0., 2., -5.), vector(0., 0., 1.), 0),
            (point(0., 1., -5.), vector(0., 0., 1.), 0),
            (point(0., 1.5, -2.), vector(0., 0., 1.), 2),
        ];
        for case in cases {
            let cyl = CylinderBuilder::new().max(2.).min(1.).build();
            let r = Ray::new(case.0, case.1.norm());
            let xs = cyl.local_intersect(r);
            assert_eq!(xs.len(), case.2)
        }
    }

    #[test]
    fn intersecting_the_caps_of_a_close_cylinder() {
        let cases = vec![
            (point(0., 3., 0.), vector(0., -1., 0.), 2),
            (point(0., 3., -2.), vector(0., -1., 2.), 2),
            (point(0., 4., -2.), vector(0., -1., 1.), 2),
            (point(0., 0., -2.), vector(0., 1., 2.), 2),
            (point(0., -1., -2.), vector(0., 1., 1.), 2),
        ];
        for case in cases {
            let cyl = CylinderBuilder::new().min(1.).max(2.).closed(true).build();
            assert_eq!(
                cyl.local_intersect(Ray::new(case.0, case.1.norm())).len(),
                case.2
            )
        }
    }

    #[test]
    fn normal_closed_cylinder() {
        let cases = vec![
            (point(0., 1., 0.), vector(0., -1., 0.)),
            (point(0.5, 1., 0.), vector(0., -1., 0.)),
            (point(0., 1., 0.5), vector(0., -1., 0.)),
            (point(0., 2., 0.), vector(0., 1., 0.)),
            (point(0.5, 2., 0.), vector(0., 1., 0.)),
            (point(0., 2., 0.5), vector(0., 1., 0.)),
        ];
        for case in cases {
            let cyl = CylinderBuilder::new().min(1.).max(2.).closed(true).build();
            assert_eq!(cyl.local_normal_at(&case.0), case.1);
        }
    }
}
