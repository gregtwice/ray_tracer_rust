use std::f64::INFINITY;

use crate::{
    intersection::Intersection,
    object::LocalIntersect,
    ray::Ray,
    tuple::{vector, Tuple},
    util::{max3, EPSILON},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cube;

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

    #[test]
    fn ray_intersects_cube() {
        struct Case {
            name: &'static str,
            origin: Tuple,
            direction: Tuple,
            t1: f64,
            t2: f64,
        }
        impl Case {
            fn new(name: &'static str, origin: Tuple, direction: Tuple, t1: f64, t2: f64) -> Self {
                Self {
                    name,
                    origin,
                    direction,
                    t1,
                    t2,
                }
            }
        }
        let c = Cube;
        let cases = [
            Case::new("+x", point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0),
            Case::new("-x", point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0),
            Case::new("+y", point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0),
            Case::new("-y", point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0),
            Case::new("+z", point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0),
            Case::new("-z", point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
            Case::new("in", point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0),
        ];

        for case in cases {
            let r = Ray::new(case.origin, case.direction);
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2, "error with cube case {}", case.name);
            assert_eq!(xs[0], case.t1);
            assert_eq!(xs[1], case.t2);
        }
    }
    #[test]
    fn ray_misses_cube() {
        let c = Cube;
        struct Case {
            o: Tuple,
            d: Tuple,
        }
        impl Case {
            fn new(o: Tuple, d: Tuple) -> Self {
                Self { o, d }
            }
        }
        let cases = [
            Case::new(point(2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
            Case::new(point(0.0, 2.0, 0.0), vector(0.8018, 0.2673, 0.5345)),
            Case::new(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673)),
            Case::new(point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
            Case::new(point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0)),
            Case::new(point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0)),
        ];

        for case in cases {
            assert_eq!(c.local_intersect(Ray::new(case.o, case.d)).len(), 0)
        }
    }

    #[test]
    fn normal_surface_cube() {
        struct Case {
            point: Tuple,
            expect: Tuple,
        }
        impl Case {
            fn new(o: Tuple, d: Tuple) -> Self {
                Self {
                    point: o,
                    expect: d,
                }
            }
        }
        let cases = [
            Case::new(point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0)),
            Case::new(point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0)),
            Case::new(point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0)),
            Case::new(point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0)),
            Case::new(point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0)),
            Case::new(point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0)),
            Case::new(point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0)),
            Case::new(point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0)),
        ];
        let c = Cube;
        for case in cases {
            assert_eq!(c.local_normal_at(&case.point), case.expect)
        }
    }
}
