use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod ray;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 256;

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3([0.0, 0.0, -1.0]),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3([0.0, -100.5, -1.0]),
            radius: 100.0,
        }),
    ];

    let camera = Camera::new(aspect_ratio, image_width, 100);
    let image = camera.render(&world);

    image.save("temp.png").unwrap();
}
