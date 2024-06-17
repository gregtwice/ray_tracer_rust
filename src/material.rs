use crate::{color::Color, lights::Light, tuple::Tuple};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: Light,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        // combine light and material color
        let effective_color = self.color * light.intensity;
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
        let result = Material::default().lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
