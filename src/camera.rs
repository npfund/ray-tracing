use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use image::RgbImage;
use rand::Rng;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Vec3([0.0, 0.0, 0.0]);

        let viewport_u = Vec3([viewport_width, 0.0, 0.0]);
        let viewport_v = Vec3([0.0, -viewport_height, 0.0]);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            camera_center - Vec3([0.0, 0.0, focal_length]) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center: Vec3([0.0, 0.0, 0.0]),
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &[Box<dyn Hittable>]) -> RgbImage {
        let mut image = RgbImage::new(self.image_width, self.image_height);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let mut color = Vec3([0.0, 0.0, 0.0]);
            for _ in 0..self.samples_per_pixel {
                let temp = self.get_ray(x, y).color(world);
                color += temp;
            }

            *pixel = (color / self.samples_per_pixel as f64).into();
        }

        image
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (x as f64 + offset[0]) * self.pixel_delta_u
            + (y as f64 + offset[1]) * self.pixel_delta_v;
        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray {
            origin,
            direction,
        }
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3([rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0])
    }
}
