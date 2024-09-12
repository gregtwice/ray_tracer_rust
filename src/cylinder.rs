use core::f64;

use rayon::vec;

use crate::{object::LocalIntersect, tuple::vector, util::EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
    closed: bool,
}

pub struct CylinderBuilder {
    cylinder: Cylinder,
}
impl CylinderBuilder {
    fn new() -> Self {
        Self {
            cylinder: Cylinder::default(),
        }
    }

    fn min(mut self, min: f64) -> Self {
        self.cylinder.min = min;
        self
    }

    fn max(mut self, max: f64) -> Self {
        self.cylinder.max = max;
        self
    }
    fn closed(mut self, closed: bool) -> Self {
        self.cylinder.closed = closed;
        self
    }

    fn build(self) -> Cylinder {
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
        let a = r.direction.x.powi(2) + r.direction.z.powi(2);
        if a <= EPSILON {
            return vec![];
        }

        let b = 2. * r.origin.x * r.direction.x + 2. * r.origin.z * r.direction.z;
        let c = r.origin.x.powi(2) + r.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4. * a * c;
        if disc < 0. {
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        return vec![t0, t1];
    }

    fn local_normal_at(&self, object_point: &crate::tuple::Tuple) -> crate::tuple::Tuple {
        vector(object_point.x, 0.0, object_point.z)
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
}
