use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    object::Shape,
    transformations::view_transform,
    tuple::{point, vector},
    world::World,
};

#[test]
fn cube_renders_on_canvas() {
    let mut world = World::ch7_default();
    let cube = Shape::cube();
    world.objects.clear();
    world.objects.push(cube);

    let mut camera = Camera::new(500, 250, PI / 3.0);
    camera.set_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 0.5, 0.0),
        vector(0.0, 1.0, 0.0),
    ));
    let image = camera.render(world);
    image.save_ppm("/home/gtwice/rust/ray-tracer/cube.ppm");
}
