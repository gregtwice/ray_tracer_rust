use std::f64::consts::{FRAC_PI_2, PI};

use ray_tracer::{
    camera::Camera,
    color::Color,
    object::Shape,
    pattern::Pattern,
    transformations::{rot_x, rot_y, scaling, translation, view_transform},
    tuple::{point, vector},
    world::World,
};

fn main() {
    let mut world = World::ch7_default();
    let mut floor = Shape::plane();
    floor.material.shininess = 20.0;
    floor.material.specular = 0.0;
    floor.material.ambient = 0.0;

    let backdrop = Shape::plane()
        .with_transform(rot_x(FRAC_PI_2).translation(0.0, 0.0, 5.0))
        .with_pattern(
            Pattern::stripped(Color::new(0.0, 1.0, 0.0), Color::new(0.0, 0.0, 1.0))
                .with_transform(rot_y(FRAC_PI_2)),
        );

    let mut middle = Shape::sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    floor.material.reflective = 1.0;

    let mut right = Shape::sphere().with_pattern(
        Pattern::gradient(Color::new(0.6, 0.6, 1.0), Color::new(1.0, 0.5, 0.5))
            .with_transform(scaling(0.5, 0.5, 0.5).rot_x(FRAC_PI_2)),
    );
    right.transform = scaling(0.5, 0.5, 0.5).translation(1.5, 0.5, -0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::sphere();
    left.transform = scaling(0.33, 0.33, 0.33).translation(-1.5, 0.0, -0.75);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    world.objects.clear();
    world.objects.push(left);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(floor);
    world.objects.push(backdrop);

    let mut camera = Camera::new(500, 250, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 0.5, 0.0),
        vector(0.0, 1.0, 0.0),
    );
    let image = camera.render(world);
    image.save_ppm("reflection.ppm");
}