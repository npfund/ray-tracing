use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

fn main() {
    let material_ground = Lambertian {
        albedo: Vec3([0.8, 0.8, 0.0]),
    };
    let material_center = Lambertian {
        albedo: Vec3([0.1, 0.2, 0.5]),
    };
    let material_left = Dielectric {
        refraction_index: 1.5,
    };
    let material_bubble = Dielectric {
        refraction_index: 1.0 / 1.5,
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
            center: Vec3([-1.0, 0.0, -1.0]),
            radius: 0.4,
            material: Box::new(material_bubble),
        }),
        Box::new(Sphere {
            center: Vec3([1.0, 0.0, -1.0]),
            radius: 0.5,
            material: Box::new(material_right),
        }),
    ];

    // let r = (f64::PI() / 4.0).cos();
    // let material_left = Lambertian {
    //     albedo: Vec3::z(1.0),
    // };
    //
    // let material_right = Lambertian {
    //     albedo: Vec3::x(1.0),
    // };
    //
    // let world: Vec<Box<dyn Hittable>> = vec![
    //     Box::new(Sphere {
    //         center: Vec3([-r, 0.0, -1.0]),
    //         radius: r,
    //         material: Box::new(material_left),
    //     }),
    //     Box::new(Sphere {
    //         center: Vec3([r, 0.0, -1.0]),
    //         radius: r,
    //         material: Box::new(material_right),
    //     }),
    // ];

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Vec3([-2.0, 2.0, 1.0]),
        Vec3([0.0, 0.0, -1.0]),
        Vec3([0.0, 1.0, 0.0]),
    );
    let image = camera.render(&world);

    image.save("temp.png").unwrap();
}
