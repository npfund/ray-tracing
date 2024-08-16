use crate::bvh::Node;
use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::texture::{Checker, Image, Noise, SolidColor};
use crate::vec3::Vec3;
use clap::Parser;
use image::RgbImage;
use rand::Rng;
use std::f64::consts::PI;

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod texture;
mod vec3;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    scene: String,
    #[arg(long, default_value = "temp.png")]
    file: String,
}

fn main() {
    let args = Args::parse();
    let image = match args.scene.as_str() {
        "triplet" => triplet(),
        "bouncing" => bouncing_final(),
        "redblue" => redblue(),
        "checkered" => checkered(),
        "earth" => earth(),
        "perlin" => perlin(),
        _ => panic!("unknown scene"),
    };

    image.save(args.file).unwrap();
}

fn triplet() -> RgbImage {
    let material_ground = Lambertian {
        texture: SolidColor::new(Vec3([0.8, 0.8, 0.0])),
    };
    let material_center = Lambertian {
        texture: SolidColor::new(Vec3([0.1, 0.2, 0.5])),
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
        Box::new(Sphere::new(
            Vec3([0.0, -100.5, -1.0]),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(Vec3([0.0, 0.0, -1.2]), 0.5, material_center)),
        Box::new(Sphere::new(Vec3([-1.0, 0.0, -1.0]), 0.5, material_left)),
        Box::new(Sphere::new(Vec3([-1.0, 0.0, -1.0]), 0.4, material_bubble)),
        Box::new(Sphere::new(Vec3([1.0, 0.0, -1.0]), 0.5, material_right)),
    ];

    let world = Node::from_list(world);

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        90.0,
        Vec3([-2.0, 2.0, 1.0]),
        Vec3([0.0, 0.0, -1.0]),
        Vec3([0.0, 1.0, 0.0]),
        10.0,
        3.4,
    );

    camera.render(&world)
}

fn redblue() -> RgbImage {
    let r = (PI / 4.0).cos();
    let material_left = Lambertian {
        texture: SolidColor::new(Vec3::z(1.0)),
    };

    let material_right = Lambertian {
        texture: SolidColor::new(Vec3::x(1.0)),
    };

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3([-r, 0.0, -1.0]), r, material_left)),
        Box::new(Sphere::new(Vec3([r, 0.0, -1.0]), r, material_right)),
    ];

    let world = Node::from_list(world);

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        90.0,
        Vec3([0.0, 0.0, 1.0]),
        Vec3([0.0, 0.0, 0.0]),
        Vec3([0.0, 1.0, 0.0]),
        0.0,
        10.0,
    );

    camera.render(&world)
}

fn bouncing_final() -> RgbImage {
    let ground_material = Lambertian {
        texture: Checker::new(
            0.32,
            SolidColor::new(Vec3([0.2, 0.3, 0.1])),
            SolidColor::new(Vec3([0.9, 0.9, 0.9])),
        ),
    };

    let mut world: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(
        Vec3([0.0, -1000.0, 0.0]),
        1000.0,
        ground_material,
    ))];

    let mut rand = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let mat = rand.gen::<f64>();
            let center = Vec3([
                a as f64 + 0.9 * rand.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rand.gen::<f64>(),
            ]);

            if (center - Vec3([4.0, 0.2, 0.0])).length() > 0.9 {
                if mat < 0.9 {
                    let material = Lambertian {
                        texture: SolidColor::new(Vec3::random() * Vec3::random()),
                    };
                    let end = center + Vec3([0.0, rand.gen(), 0.0]);
                    world.push(Box::new(Sphere::moving(center, end, 0.2, material)));
                } else if mat < 0.95 {
                    let material = Metal {
                        albedo: Vec3::random_within(0.5, 1.0),
                        fuzz: rand.gen::<f64>(),
                    };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Dielectric {
                        refraction_index: 1.5,
                    };
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material_1 = Dielectric {
        refraction_index: 1.5,
    };
    world.push(Box::new(Sphere::new(
        Vec3([0.0, 1.0, 0.0]),
        1.0,
        material_1,
    )));

    let material_2 = Lambertian {
        texture: SolidColor::new(Vec3([0.4, 0.2, 0.1])),
    };
    world.push(Box::new(Sphere::new(
        Vec3([-4.0, 1.0, 0.0]),
        1.0,
        material_2,
    )));

    let material_3 = Metal {
        albedo: Vec3([0.7, 0.6, 0.5]),
        fuzz: 0.0,
    };
    world.push(Box::new(Sphere::new(
        Vec3([4.0, 1.0, 0.0]),
        1.0,
        material_3,
    )));

    let world = Node::from_list(world);

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Vec3([13.0, 2.0, 3.0]),
        Vec3([0.0, 0.0, 0.0]),
        Vec3([0.0, 1.0, 0.0]),
        0.6,
        10.0,
    );

    camera.render(&world)
}

fn checkered() -> RgbImage {
    let ground_material = Lambertian {
        texture: Checker::new(
            0.32,
            SolidColor::new(Vec3([0.2, 0.3, 0.1])),
            SolidColor::new(Vec3([0.9, 0.9, 0.9])),
        ),
    };

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Vec3([0.0, -10.0, 0.0]),
            10.0,
            ground_material.clone(),
        )),
        Box::new(Sphere::new(Vec3([0.0, 10.0, 0.0]), 10.0, ground_material)),
    ];

    let world = Node::from_list(world);

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Vec3([13.0, 2.0, 3.0]),
        Vec3([0.0, 0.0, 0.0]),
        Vec3([0.0, 1.0, 0.0]),
        0.0,
        10.0,
    );

    camera.render(&world)
}

fn earth() -> RgbImage {
    let earth_texture = image::open("earthmap.jpg").unwrap().into_rgb8();
    let earth_surface = Lambertian {
        texture: Image::new(earth_texture),
    };
    let globe = Sphere::new(Vec3::scalar(0.0), 2.0, earth_surface);

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Vec3([0.0, 0.0, 12.0]),
        Vec3([0.0, 0.0, 0.0]),
        Vec3([0.0, 1.0, 0.0]),
        0.0,
        10.0,
    );

    camera.render(&globe)
}

fn perlin() -> RgbImage {
    let ground_material = Lambertian {
        texture: Noise::<256>::new(4.0),
    };

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Vec3([0.0, -1000.0, 0.0]),
            1000.0,
            ground_material.clone(),
        )),
        Box::new(Sphere::new(Vec3([0.0, 2.0, 0.0]), 2.0, ground_material)),
    ];

    let world = Node::from_list(world);

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Vec3([13.0, 2.0, 3.0]),
        Vec3([0.0, 0.0, 0.0]),
        Vec3([0.0, 1.0, 0.0]),
        0.0,
        10.0,
    );

    camera.render(&world)
}
