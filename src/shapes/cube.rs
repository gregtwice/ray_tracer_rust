use std::f64::INFINITY;

use crate::{
    bounds::{Bounded, BoundingBox},
    object::LocalIntersect,
    ray::Ray,
    tuple::{point, vector, Tuple},
    util::{max3, EPSILON},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cube;

impl Bounded for Cube {
    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;
    let mut tmin;
    let mut tmax;
    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * INFINITY;
        tmax = tmax_numerator * INFINITY;
    }
    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }
    return (tmin, tmax);
}

impl LocalIntersect for Cube {
    fn local_intersect(&self, r: Ray) -> Vec<f64> {
        let (xt_min, xt_max) = check_axis(r.origin.x, r.direction.x);
        let (yt_min, yt_max) = check_axis(r.origin.y, r.direction.y);
        let (zt_min, zt_max) = check_axis(r.origin.z, r.direction.z);
        let t_min = xt_min.max(yt_min).max(zt_min);
        let t_max = xt_max.min(yt_max).min(zt_max);
        if t_min > t_max {
            vec![]
        } else {
            vec![t_min, t_max]
        }
    }

    fn local_normal_at(&self, object_point: &Tuple) -> Tuple {
        let max_c = max3(
            object_point.x.abs(),
            object_point.y.abs(),
            object_point.z.abs(),
        );
        if max_c == object_point.x.abs() {
            vector(object_point.x, 0.0, 0.0)
        } else if max_c == object_point.y.abs() {
            vector(0.0, object_point.y, 0.0)
        } else {
            vector(0.0, 0.0, object_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        tuple::{point, vector, Tuple},
    };

    use super::*;
    use test_case::test_case;

    #[test_case(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0; "positive x")]
    #[test_case(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0 ; "negative x")]
    #[test_case(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0 ; "positive y")]
    #[test_case(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0 ; "negative y")]
    #[test_case(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0 ; "positive z")]
    #[test_case(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0 ; "negative z")]
    #[test_case(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0 ; "inside")]
    fn ray_intersects_cube(origin: Tuple, direction: Tuple, t1: f64, t2: f64) {
        let c = Cube;

        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], t1);
        assert_eq!(xs[1], t2);
    }

    #[test_case(point(2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018))]
    #[test_case(point(0.0, 2.0, 0.0), vector(0.8018, 0.2673, 0.5345))]
    #[test_case(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673))]
    #[test_case(point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0))]
    #[test_case(point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0))]
    #[test_case(point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0))]
    fn ray_misses_cube(origin: Tuple, dest: Tuple) {
        let c = Cube;
        assert_eq!(c.local_intersect(Ray::new(origin, dest)).len(), 0)
    }

    #[test_case(point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0) ; "normal")]
    #[test_case(point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0))]
    #[test_case(point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0))]
    #[test_case(point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0))]
    #[test_case(point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0))]
    #[test_case(point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0))]
    #[test_case(point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0); "px")]
    #[test_case(point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0); "nx")]
    fn normal_surface_cube(point: Tuple, normal: Tuple) {
        let c = Cube;
        assert_eq!(c.local_normal_at(&point), normal)
    }
}
