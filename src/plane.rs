use crate::{object::LocalIntersect, tuple::vector, util::EPSILON};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plane;

impl LocalIntersect for Plane {
    fn local_intersect(&self, r: crate::ray::Ray) -> Vec<f64> {
        if r.direction.y.abs() < EPSILON {
            vec![]
        } else {
            vec![-r.origin.y / r.direction.y]
        }
    }

    fn local_normal_at(&self, _: &crate::tuple::Tuple) -> crate::tuple::Tuple {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersection::Intersectable, object::Shape, ray::Ray, tuple::point};

    use super::*;

    #[test]
    fn normal_constant_everywhere() {
        let p = Shape::plane();
        let n1 = p.normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_parallel_ray() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        assert_eq!(p.intersects(r).data().len(), 0)
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        assert_eq!(p.intersects(r).data().len(), 0)
    }

    #[test]
    fn intersect_with_ray_from_above() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersects(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].time, 1.0);
        assert_eq!(xs[0].object, &p);
    }

    #[test]
    fn intersect_with_ray_from_below() {
        let p = Shape::plane();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersects(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].time, 1.0);
        assert_eq!(xs[0].object, &p);
    }
}
