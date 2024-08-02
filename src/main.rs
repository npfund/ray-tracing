use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Lambertian, Metal};
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 256;

    let material_ground = Lambertian {
        albedo: Vec3([0.8, 0.8, 0.0]),
    };
    let material_center = Lambertian {
        albedo: Vec3([0.1, 0.2, 0.5]),
    };
    let material_left = Metal {
        albedo: Vec3([0.8, 0.8, 0.8]),
        fuzz: 0.3,
    };
    let material_right = Metal {
        albedo: Vec3([0.8, 0.6, 0.2]),
        fuzz: 1.0,
    };

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3([0.0, -100.5, -1.0]),
            radius: 100.0,
            material: Box::new(material_ground),
        }),
        Box::new(Sphere {
            center: Vec3([0.0, 0.0, -1.2]),
            radius: 0.5,
            material: Box::new(material_center),
        }),
        Box::new(Sphere {
            center: Vec3([-1.0, 0.0, -1.0]),
            radius: 0.5,
            material: Box::new(material_left),
        }),
        Box::new(Sphere {
            center: Vec3([1.0, 0.0, -1.0]),
            radius: 0.5,
            material: Box::new(material_right),
        }),
    ];

    let camera = Camera::new(aspect_ratio, image_width, 100, 50);
    let image = camera.render(&world);

    image.save("temp.png").unwrap();
}
