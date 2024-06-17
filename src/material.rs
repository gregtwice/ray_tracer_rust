use crate::{color::Color, lights::Light, object::Shape, pattern::Pattern, tuple::Tuple};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: Light,
        shape: Shape,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        // combine light and material color
        let effective_color = match self.pattern {
            Some(p) => p.pattern_at_shape(shape, point),
            None => self.color,
        } * light.intensity;
        // find direction to the light source
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }
        let lightv = (light.position - point).norm();
        let ligtht_dot_normal = lightv.dot(normalv);
        let diffuse;
        let specular;
        if ligtht_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * ligtht_dot_normal;

            let reflect_v = (-lightv).reflect(&normalv);
            let relect_dot_eye = reflect_v.dot(eyev);
            if relect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = relect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use crate::{
        color::Color,
        lights::Light,
        object::Shape,
        pattern::Pattern,
        tuple::{point, vector},
    };

    use super::Material;

    #[test]
    fn lighting_surface_in_shadow() {
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = Material::default().lighting(
            light,
            Shape::sphere(),
            position,
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut material = Material::default();
        material.pattern = Some(Pattern::stripped(Color::white(), Color::black()));
        material.ambient = 1.0;
        material.diffuse = 0.0;
        material.specular = 0.0;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::white());
        let c1 = material.lighting(
            light,
            Shape::sphere(),
            point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = material.lighting(
            light,
            Shape::sphere(),
            point(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }
}
