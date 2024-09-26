use core::f64;

use crate::{
    object::LocalIntersect,
    ray::Ray,
    tuple::vector,
    util::{flt_eq, EPSILON},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cone {
    min: f64,
    max: f64,
    closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }
}

pub struct ConeBuilder {
    cone: Cone,
}

impl ConeBuilder {
    pub fn new() -> Self {
        Self {
            cone: Cone::default(),
        }
    }
    pub fn closed(mut self, closed: bool) -> Self {
        self.cone.closed = closed;
        self
    }
    pub fn min(mut self, min: f64) -> Self {
        self.cone.min = min;
        self
    }
    pub fn max(mut self, max: f64) -> Self {
        self.cone.max = max;
        self
    }
    pub fn build(self) -> Cone {
        self.cone
    }
}

impl Cone {
    fn check_cap(r: Ray, t: f64, y: f64) -> bool {
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;

        return (x * x + z * z) <= y.abs();
    }

    fn intersect_caps(&self, r: Ray, xs: &mut Vec<f64>) {
        if !self.closed || r.direction.y.abs() < EPSILON {
            return;
        }
        let t = (self.min - r.origin.y) / r.direction.y;
        if Self::check_cap(r, t, self.min) {
            xs.push(t);
        }

        let t = (self.max - r.origin.y) / r.direction.y;
        if Self::check_cap(r, t, self.max) {
            xs.push(t);
        }
    }
}

impl LocalIntersect for Cone {
    fn local_intersect(&self, r: crate::ray::Ray) -> Vec<f64> {
        let mut xs = Vec::new();
        let a = r.direction.x.powi(2) - r.direction.y.powi(2) + r.direction.z.powi(2);
        let b = 2.
            * (r.origin.x * r.direction.x - r.origin.y * r.direction.y
                + r.origin.z * r.direction.z);
        let c = r.origin.x.powi(2) - r.origin.y.powi(2) + r.origin.z.powi(2);
        if a.abs() < EPSILON && b.abs() < EPSILON {
            return xs;
        }
        if a.abs() < EPSILON {
            xs.push(-c / (2. * b));
            self.intersect_caps(r, &mut xs);
            return xs;
        }

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
            xs.push(t0);
        }
        let y1 = r.origin.y + t1 * r.direction.y;
        if self.min < y1 && y1 < self.max {
            xs.push(t1);
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
            let mut y = f64::sqrt(object_point.x.powi(2) + object_point.z.powi(2));
            if object_point.y > 0.0 {
                y = -y;
            }
            vector(object_point.x, y, object_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn intersect_cone_() {
        let cases = vec![
            (point(0., 0., -5.), vector(0., 0., 1.), 5., 5.),
            (point(0., 0., -5.), vector(1., 1., 1.), 8.66025, 8.66025),
            (point(1., 1., -5.), vector(-0.5, -1., 1.), 4.55006, 49.44994),
        ];
        for case in cases {
            let c = Cone::default();
            let r = Ray::new(case.0, case.1.norm());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert!(flt_eq(xs[0], case.2));
            assert!(flt_eq(xs[1], case.3));
        }
    }
    #[test]
    fn cone_ray_parallel() {
        let c = Cone::default();
        let r = Ray::new(point(0., 0., -1.), vector(0., 1., 1.).norm());
        let xs = c.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert!(flt_eq(xs[0], 0.35355))
    }

    #[test]
    fn intersect_cone_end_caps() {
        let cases = vec![
            (point(0., 0., -5.), vector(0., 1., 0.), 0),
            (point(0., 0., -0.25), vector(0., 1., 1.), 2),
            (point(0., 0., -0.25), vector(0., 1., 0.), 4),
        ];
        for case in cases {
            let r = Ray::new(case.0, case.1.norm());
            let c = ConeBuilder::new().min(-0.5).max(0.5).closed(true).build();
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), case.2);
        }
    }
    #[test]
    fn normal_cone() {
        let cases = [
            (point(0., 0., 0.), vector(0., 0., 0.)),
            (point(1., 1., 1.), vector(1., -f64::sqrt(2.), 1.)),
            (point(-1., -1., 0.), vector(-1., 1., 0.)),
        ];
        for case in cases {
            let c = Cone::default();
            let n = c.local_normal_at(&case.0);
            assert_eq!(n, case.1);
        }
    }
}
