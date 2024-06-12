use std::io::Write;

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    fn to_xy(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        let coords = self.to_xy(x, y);
        self.pixels[coords] = color
    }

    pub fn write_pixel_f(&mut self, x: f64, y: f64, color: Color) {
        assert!((x as usize) < self.width);
        assert!((y as usize) < self.height);
        let coords = self.to_xy(x as usize, y as usize);
        self.pixels[coords] = color
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[self.to_xy(x, y)]
    }

    pub fn save_ppm(&self, filename: &str) {
        let mut image = std::fs::File::create(filename).expect("wtf");
        image.write("P3\n".as_bytes()).unwrap();
        image
            .write(format!("{} {}\n", self.width, self.height).as_bytes())
            .unwrap();
        image.write("255\n".as_bytes()).unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.pixel_at(x, y);
                image
                    .write(
                        format!(
                            "{} {} {}\n",
                            (c.r() * 255.0).floor(),
                            (c.g() * 255.0).floor(),
                            (c.b() * 255.0).floor()
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }
        image.write("\n".as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{
        color::Color,
        matrix::Mat4,
        tuple::{point, vector},
    };

    use super::Canvas;

    #[test]
    fn test_coords() {
        let mut canvas = Canvas::new(40, 40);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(0, 0, red);
        dbg!(&canvas);
        assert!(canvas.pixel_at(0, 0) == red);

        canvas.save_ppm("place_pixel.ppm");
    }

    #[test]
    fn grav() {
        let mut canvas = Canvas::new(900, 600);
        let start = point(0.0, 1.0, 0.0);
        let mut velocity = vector(1.0, 1.8, 0.0).norm() * 11.25;
        let gravity = vector(0.0, -0.1, 0.0);
        let wind = vector(-0.01, 0.0, 0.0);
        let mut current = start;
        loop {
            if current.x as usize > canvas.width || current.y < 0.0 {
                break;
            }
            velocity += wind + gravity;
            current += velocity;
            canvas.write_pixel_f(current.x, 550.0 - current.y, Color::new(1.0, 0.0, 0.0));
        }
        canvas.save_ppm("curves.ppm");
    }

    #[test]
    fn test_clock() {
        let center = point(0.0, 0.0, 0.0);
        let twelve = point(0.0, 0.0, 1.0);
        let mut canvas = Canvas::new(100, 100);

        for i in 0..12 {
            let t = Mat4::identity().rot_y(PI / 6.0 * i as f64);

            let ptw = t * (twelve);
            let scaling = 30.0;
            let ptw =
                ptw * scaling + point((canvas.width / 2) as f64, 0.0, (canvas.height / 2) as f64);
            canvas.write_pixel_f(ptw.x, ptw.z, Color::new(1.0, 1.0, 0.0));
        }
        canvas.save_ppm("clock.ppm");
    }
}
