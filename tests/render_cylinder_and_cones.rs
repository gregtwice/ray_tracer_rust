use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    object::Shape,
    transformations::{rot_y, scaling, view_transform},
    tuple::{point, vector},
    world::World,
};

#[test]
fn can_render_all_shapes() {
    let mut world = World::ch7_default();
    let cyl = Shape::cone(-1., 0., true).with_transform(rot_y(PI / 4.));
    world.objects.clear();
    world.objects.push(cyl);
    let mut camera = Camera::new(500, 250, PI / 3.0);
    camera.set_transform(view_transform(
        point(0.0, 1.5, -10.0),
        point(0.0, 0.5, 0.0),
        vector(0.0, 1.0, 0.0),
    ));
    let image = camera.render(world);
    image.save_ppm("/home/gtwice/rust/ray-tracer/cyl.ppm");
}
