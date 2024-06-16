use std::vec;

use crate::{
    color::Color,
    intersection::{self, Computations, Intersectable, Intersections},
    lights::Light,
    object::Object,
    sphere::Sphere,
    transformations::scaling,
    tuple::point,
};

pub struct World {
    lights: Vec<Light>,
    objects: Vec<Object>,
}

impl World {
    pub fn ch7_default() -> Self {
        let light_position = point(-10.0, 10.0, -10.0);
        let light_color = Color::new(1.0, 1.0, 1.0);
        let light = Light::new(light_position, light_color);
        let mut s1 = Sphere::new();
        let mut s2 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        Self {
            lights: vec![light],
            objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
        }
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        comps
            .i
            .object
            .material()
            .lighting(self.lights[0], comps.point, comps.eye_v, comps.normal_v)
    }

    pub fn intersects(&self, r: crate::ray::Ray) -> intersection::Intersections {
        let mut i = self
            .objects
            .iter()
            .map(|o| o.intersects(r).into_inner())
            .flatten()
            .collect::<Vec<_>>();
        i.sort_by(|a, b| a.time.total_cmp(&b.time));
        Intersections::new(i)
    }
}

#[cfg(test)]
mod tests {
    use intersection::Intersection;

    use crate::{ray::Ray, tuple::vector};

    use super::*;
    #[test]
    fn intersect_world_with_ray() {
        let w = World::ch7_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersects(r);
        assert_eq!(xs.data().len(), 4);
        assert_eq!(xs.data()[0].time, 4.0);
        assert_eq!(xs.data()[1].time, 4.5);
        assert_eq!(xs.data()[2].time, 5.5);
        assert_eq!(xs.data()[3].time, 6.0);
    }

    #[test]
    fn shading_intersection() {
        let w = World::ch7_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[0];
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_intersection_inside() {
        let mut w = World::ch7_default();
        w.lights = vec![Light::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0))];
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let i = Intersection::new(0.5, s);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498))
    }
}
