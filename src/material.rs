use crate::{
    color::{self, Color},
    lights::Light,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambiant: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambiant: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(&self, light: Light, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        // combine light and material color
        let effective_color = self.color * light.intensity;
        // find direction to the light source
        let lightv = (light.position - point).norm();
        let ambiant = effective_color * self.ambiant;
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
        ambiant + diffuse + specular
    }
}
