use rand::Rng;
use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

fn main() {
    // let material_ground = Lambertian {
    //     albedo: Vec3([0.8, 0.8, 0.0]),
    // };
    // let material_center = Lambertian {
    //     albedo: Vec3([0.1, 0.2, 0.5]),
    // };
    // let material_left = Dielectric {
    //     refraction_index: 1.5,
    // };
    // let material_bubble = Dielectric {
    //     refraction_index: 1.0 / 1.5,
    // };
    // let material_right = Metal {
    //     albedo: Vec3([0.8, 0.6, 0.2]),
    //     fuzz: 1.0,
    // };
    //
    // let world: Vec<Box<dyn Hittable>> = vec![
    //     Box::new(Sphere {
    //         center: Vec3([0.0, -100.5, -1.0]),
    //         radius: 100.0,
    //         material: Box::new(material_ground),
    //     }),
    //     Box::new(Sphere {
    //         center: Vec3([0.0, 0.0, -1.2]),
    //         radius: 0.5,
    //         material: Box::new(material_center),
    //     }),
    //     Box::new(Sphere {
    //         center: Vec3([-1.0, 0.0, -1.0]),
    //         radius: 0.5,
    //         material: Box::new(material_left),
    //     }),
    //     Box::new(Sphere {
    //         center: Vec3([-1.0, 0.0, -1.0]),
    //         radius: 0.4,
    //         material: Box::new(material_bubble),
    //     }),
    //     Box::new(Sphere {
    //         center: Vec3([1.0, 0.0, -1.0]),
    //         radius: 0.5,
    //         material: Box::new(material_right),
    //     }),
    // ];

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

    let ground_material = Lambertian {
        albedo: Vec3::scalar(0.5),
    };

    let mut world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Vec3([0.0, -1000.0, 0.0]),
            radius: 1000.0,
            material: Box::new(ground_material),
        })
    ];

    let mut rand = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let mat = rand.gen::<f64>();
            let center = Vec3([a as f64 + 0.9 * rand.gen::<f64>(), 0.2, b as f64 + 0.9 * rand.gen::<f64>()]);

            if (center - Vec3([4.0, 0.2, 0.0])).length() > 0.9 {
                let material: Box<dyn Material> = if mat < 0.9 {
                    Box::new(Lambertian {
                        albedo: Vec3::random() * Vec3::random(),
                    })
                } else if mat < 0.95 {
                    Box::new(Metal {
                        albedo: Vec3::random_within(0.5, 1.0),
                        fuzz: rand.gen::<f64>(),
                    })
                } else {
                    Box::new(Dielectric {
                        refraction_index: 1.5,
                    })
                };

                world.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }));
            }
        }
    }

    let material_1 = Dielectric {
        refraction_index: 1.5,
    };
    world.push(Box::new(Sphere {
        center: Vec3([0.0, 1.0, 0.0]),
        radius: 1.0,
        material: Box::new(material_1),
    }));

    let material_2 = Lambertian {
        albedo: Vec3([0.4, 0.2, 0.1]),
    };
    world.push(Box::new(Sphere {
        center: Vec3([-4.0, 1.0, 0.0]),
        radius: 1.0,
        material: Box::new(material_2),
    }));

    let material_3 = Metal {
        albedo: Vec3([0.7, 0.6, 0.5]),
        fuzz: 0.0,
    };
    world.push(Box::new(Sphere {
        center: Vec3([4.0, 1.0, 0.0]),
        radius: 1.0,
        material: Box::new(material_3),
    }));

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Vec3([13.0, 2.0, 3.0]),
        Vec3([0.0, 0.0, 0.0]),
        Vec3([0.0, 1.0, 0.0]),
        0.6,
        10.0,
    );
    let image = camera.render(&world);

    image.save("temp.png").unwrap();
}
