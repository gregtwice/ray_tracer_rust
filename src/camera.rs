use crate::{
    canvas::Canvas,
    matrix::{Mat4, MatBase},
    ray::Ray,
    tuple::point,
    world::World,
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f64,
    pub transform: Mat4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let (half_height, half_width, pixel_size) = Self::pixel_size(hsize, vsize, fov);
        Self {
            hsize,
            vsize,
            fov,
            transform: Mat4::identity(),
            pixel_size: pixel_size,
            half_height,
            half_width,
        }
    }

    fn pixel_size(hsize: usize, vsize: usize, fov: f64) -> (f64, f64, f64) {
        let half_view = f64::tan(fov / 2.0);
        let aspect_ratio = hsize as f64 / vsize as f64;
        let half_width;
        let half_height;

        if aspect_ratio >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect_ratio;
        } else {
            half_width = half_view * aspect_ratio;
            half_height = half_view;
        }
        (half_height, half_width, (half_width * 2.0) / hsize as f64)
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x = x as f64;
        let y = y as f64;
        let offset_x = (x + 0.5) * self.pixel_size;
        let offset_y = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        let pixel = (self.transform.inverse()) * point(world_x, world_y, -1.0);
        let origin = (self.transform.inverse()) * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).norm();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let color = world.color_at(r);
                image.write_pixel(x, y, color);
            }
        }
        image
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        color::Color,
        transformations::{translation, view_transform},
        tuple::{point, vector},
        util::flt_eq,
        world::World,
    };

    use super::Camera;

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(flt_eq(c.pixel_size, 0.01))
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(flt_eq(c.pixel_size, 0.01))
    }

    #[test]
    fn ray_center_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_corner_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = translation(0.0, -2.0, 5.0).rot_y(PI / 4.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::ch7_default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);
        let image = c.render(w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855))
    }
}
