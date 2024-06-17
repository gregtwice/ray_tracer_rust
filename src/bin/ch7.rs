use std::f64::consts::{FRAC_PI_2, PI};

use ray_tracer::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    intersection::Intersectable,
    object::{Object, Shape},
    sphere::Sphere,
    transformations::{rot_x, scaling, translation, view_transform},
    tuple::{point, vector},
    world::World,
};

fn main() {
    let mut world = World::ch7_default();
    let mut floor = Shape::sphere();
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Shape::sphere();
    left_wall.transform = scaling(10.0, 0.01, 10.0)
        .rot_x(FRAC_PI_2)
        .rot_y(-PI / 4.0)
        .translation(0.0, 0.0, 5.0);
    left_wall.material = floor.material;

    let mut right_wall = Shape::sphere();
    right_wall.transform = scaling(10.0, 0.01, 10.0)
        .rot_x(FRAC_PI_2)
        .rot_y(PI / 4.0)
        .translation(0.0, 0.0, 5.0);
    right_wall.material = floor.material;

    let mut middle = Shape::sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::sphere();
    right.transform = scaling(0.5, 0.5, 0.5).translation(1.5, 0.5, -0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::sphere();
    left.transform = scaling(0.33, 0.33, 0.33).translation(-1.5, 0.33, -0.75);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    let mut camera = Camera::new(1000, 500, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );
    world.objects.clear();
    world.objects.push(left);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left_wall);
    world.objects.push(floor);
    world.objects.push(right_wall);
    let image = camera.render(world);
    image.save_ppm("end_ch7.ppm");
}
