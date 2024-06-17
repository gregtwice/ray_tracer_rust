use std::f64::consts::{FRAC_PI_2, PI};

use ray_tracer::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    intersection::Intersectable,
    object::Object,
    sphere::Sphere,
    transformations::{rot_x, scaling, translation, view_transform},
    tuple::{point, vector},
    world::World,
};

fn main() {
    let mut world = World::ch7_default();
    let mut floor = Sphere::new();
    floor.set_transform(scaling(10.0, 0.01, 10.0));
    floor.material_mut().color = Color::new(1.0, 0.9, 0.9);
    floor.material_mut().specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.set_transform(
        scaling(10.0, 0.01, 10.0)
            .rot_x(FRAC_PI_2)
            .rot_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    *left_wall.material_mut() = floor.material();

    let mut right_wall = Sphere::new();
    right_wall.set_transform(
        scaling(10.0, 0.01, 10.0)
            .rot_x(FRAC_PI_2)
            .rot_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    *right_wall.material_mut() = floor.material();

    let mut middle = Sphere::new();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.material_mut().color = Color::new(0.1, 1.0, 0.5);
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Sphere::new();
    right.set_transform(scaling(0.5, 0.5, 0.5).translation(1.5, 0.5, -0.5));
    right.material_mut().color = Color::new(0.5, 1.0, 0.1);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Sphere::new();
    left.set_transform(scaling(0.33, 0.33, 0.33).translation(-1.5, 0.33, -0.75));
    left.material_mut().color = Color::new(1.0, 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;
    let mut camera = Camera::new(1000, 500, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );
    world.objects.clear();
    world.objects.push(Object::Sphere(left));
    world.objects.push(Object::Sphere(middle));
    world.objects.push(Object::Sphere(right));
    world.objects.push(Object::Sphere(left_wall));
    world.objects.push(Object::Sphere(floor));
    world.objects.push(Object::Sphere(right_wall));
    let image = camera.render(world);
    image.save_ppm("end_ch7.ppm");
}
