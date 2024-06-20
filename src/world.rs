use std::vec;

use crate::{
    color::Color,
    intersection::{self, Computations, Intersectable, Intersections},
    lights::Light,
    object::Shape,
    ray::Ray,
    transformations::scaling,
    tuple::{point, Tuple},
};

pub struct World {
    lights: Vec<Light>,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new() -> Self {
        Self {
            lights: vec![],
            objects: vec![],
        }
    }
    pub fn ch7_default() -> Self {
        let light_position = point(-10.0, 10.0, -10.0);
        let light_color = Color::new(1.0, 1.0, 1.0);
        let light = Light::new(light_position, light_color);
        let mut s1 = Shape::sphere();
        let mut s2 = Shape::sphere();

        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        Self {
            lights: vec![light],
            objects: vec![s1, s2],
        }
    }

    pub fn shade_hit(&self, comps: Computations, depth: usize) -> Color {
        let surface = comps.i.object.material.lighting(
            self.lights[0],
            comps.i.object,
            comps.over_point,
            comps.eye_v,
            comps.normal_v,
            self.is_shadowed(comps.over_point),
        );
        let reflected = self.reflect_color(comps, depth);
        surface + reflected + self.refracted_color(comps, depth)
    }

    pub fn reflect_color(&self, comps: Computations, depth: usize) -> Color {
        if depth == 0 {
            return Color::black();
        }
        if comps.i.object.material.reflective == 0.0 {
            Color::black()
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflect_v);
            let color = self.color_at(reflect_ray, depth - 1);
            color * comps.i.object.material.reflective
        }
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

    pub fn color_at(&self, r: crate::ray::Ray, depth: usize) -> Color {
        let xs = self.intersects(r);
        let hit = xs.hit();
        match hit {
            Some(h) => self.shade_hit(h.prepare_computations(r, &xs), depth),
            None => Color::black(),
        }
    }

    pub fn refracted_color(&self, comps: Computations, depth: usize) -> Color {
        if comps.i.object.material.transparency == 0.0 || depth == 0 {
            Color::black()
        } else {
            // compute snell's law
            let n_ratio = comps.n1 / comps.n2;
            let cos_i = comps.eye_v.dot(comps.normal_v);
            let sin2_t = n_ratio * n_ratio * (1.0 - (cos_i * cos_i));
            if sin2_t > 1.0 {
                Color::black()
            } else {
                let cos_t = f64::sqrt(1.0 - sin2_t);
                let direction = comps.normal_v * (n_ratio * cos_i - cos_t) - comps.eye_v * n_ratio;
                let refracted_ray = Ray::new(comps.under_point, direction);
                self.color_at(refracted_ray, depth - 1) * comps.i.object.material.transparency
            }
        }
    }

    fn is_shadowed(&self, p: Tuple) -> bool {
        let v = self.lights[0].position - p;
        let distance = v.mag();
        let direction = v.norm();
        let r = Ray::new(p, direction);
        let xs = self.intersects(r);
        let h = xs.hit();
        if h.is_some_and(|h| h.time < distance) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use intersection::Intersection;

    use crate::{
        material::Material, pattern::Pattern, ray::Ray, transformations::translation,
        tuple::vector, util::MAX_REFLECTIONS,
    };

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
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let c = w.shade_hit(comps, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_intersection_inside() {
        let mut w = World::ch7_default();
        w.lights = vec![Light::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0))];
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let i = Intersection::new(0.5, s);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let c = w.shade_hit(comps, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498))
    }

    #[test]
    fn ray_misses() {
        let w = World::ch7_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(r, MAX_REFLECTIONS);
        assert_eq!(c, Color::black())
    }

    #[test]
    fn ray_hits() {
        let w = World::ch7_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(r, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn color_with_intersection_behind_the_ray() {
        let mut w = World::ch7_default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(r, MAX_REFLECTIONS);
        assert_eq!(c, w.objects[1].material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear() {
        let w = World::ch7_default();
        let p = point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shadow_when_point_behind_object() {
        let w = World::ch7_default();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::ch7_default();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn no_shadow_when_object_behind_the_point() {
        let w = World::ch7_default();
        let p = point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let mut w = World::new();

        w.lights.push(Light::new(
            point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        w.objects.push(Shape::sphere());
        let mut s2 = Shape::sphere();
        s2.set_transform(translation(0.0, 0.0, 10.0));
        w.objects.push(s2);
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let c = w.shade_hit(comps, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_on_non_relfective_surface() {
        let mut w = World::ch7_default();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        w.objects[1].material.ambient = 1.0;
        let i = Intersection::new(1.0, w.objects[1]);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let color = w.reflect_color(comps, MAX_REFLECTIONS);
        assert_eq!(color, Color::black())
    }

    #[test]
    fn reflected_color_on_reflective_surface() {
        let mut w = World::ch7_default();
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let mut p = Shape::plane().with_transform(translation(0.0, -1.0, 0.0));
        p.material.reflective = 0.5;
        w.objects.push(p);

        let i = Intersection::new(SQRT_2, p);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let color = w.reflect_color(comps, MAX_REFLECTIONS);
        assert_eq!(color, Color::new(0.19033, 0.237915, 0.142749))
    }

    #[test]
    fn reflected_color_shade_hit_on_reflective_surface() {
        let mut w = World::ch7_default();
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let mut p = Shape::plane().with_transform(translation(0.0, -1.0, 0.0));
        p.material.reflective = 0.5;
        w.objects.push(p);

        let i = Intersection::new(SQRT_2, p);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let color = w.shade_hit(comps, MAX_REFLECTIONS);
        assert_eq!(color, Color::new(0.87675, 0.92434, 0.82917))
    }

    #[test]
    fn mutually_reflective_surfaces() {
        let mut w = World::ch7_default();
        let lower = Shape::plane()
            .with_material(Material::default().reflective(1.0))
            .with_transform(translation(0.0, -1.0, 0.0));
        let upper = Shape::plane()
            .with_material(Material::default().reflective(1.0))
            .with_transform(translation(0.0, 1.0, 0.0));
        let light = Light::new(point(0.0, 0.0, 0.0), Color::white());
        w.lights.push(light);
        w.objects.push(lower);
        w.objects.push(upper);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(r, MAX_REFLECTIONS);
        assert!(c != Color::black())
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let mut w = World::ch7_default();
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let mut p = Shape::plane().with_transform(translation(0.0, -1.0, 0.0));
        p.material.reflective = 0.5;
        w.objects.push(p);

        let i = Intersection::new(SQRT_2, p);
        let comps = i.prepare_computations(r, &Intersections::new(vec![i]));
        let color = w.reflect_color(comps, 0);
        assert_eq!(color, Color::black())
    }

    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = World::ch7_default();
        let s = w.objects[0];
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![Intersection::new(4.0, s), Intersection::new(6.0, s)]);
        let comps = xs.data()[0].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = World::ch7_default();
        let s = &mut w.objects[0];
        s.material = s.material.transparency(1.0).refractive_index(1.5);
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![Intersection::new(4.0, *s), Intersection::new(6.0, *s)]);
        let comps = xs.data()[0].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 0);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn refracted_color_total_internal_reflection() {
        let mut w = World::ch7_default();
        let s = &mut w.objects[0];
        s.material = s.material.transparency(1.0).refractive_index(1.5);
        let r = Ray::new(point(0.0, 0.0, SQRT_2 / 2.0), vector(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-SQRT_2 / 2.0, *s),
            Intersection::new(SQRT_2 / 2.0, *s),
        ]);
        let comps = xs.data()[1].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::ch7_default();
        let a = &mut w.objects[0];
        a.material.ambient = 1.0;
        a.set_pattern(Pattern::test_pattern());
        let a = w.objects[0];
        let b = &mut w.objects[1];
        b.material = b.material.transparency(1.0).refractive_index(1.5);
        let r = Ray::new(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, *b),
            Intersection::new(0.4899, *b),
            Intersection::new(0.9899, a),
        ]);
        let comps = xs.data()[2].prepare_computations(r, &xs);
        let c = w.refracted_color(comps, 5);
        assert_eq!(c, Color::new(0.0, 0.998874, 0.047218));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::ch7_default();
        let floor = Shape::plane()
            .with_material(Material::default().transparency(0.5).refractive_index(1.5))
            .with_transform(translation(0.0, -1.0, 0.0));
        w.objects.push(floor);
        let ball = Shape::sphere()
            .with_material(
                Material::default()
                    .ambient(0.5)
                    .color(Color::new(1.0, 0.0, 0.0)),
            )
            .with_transform(translation(0.0, -3.5, -0.5));
        w.objects.push(ball);

        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let xs = Intersections::new(vec![Intersection::new(f64::sqrt(2.0), floor)]);
        let comps = xs.data()[0].prepare_computations(r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_eq!(c, Color::new(0.93642, 0.68642, 0.68642));
    }
}
