use image::{RgbImage};
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
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
        }
    }

    pub fn render(&self, world: &[Box<dyn Hittable>]) -> RgbImage {
        let mut image = RgbImage::new(self.image_width, self.image_height);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let pixel_center = self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
            let ray_direction = pixel_center - self.center;
            let ray = Ray {
                origin: self.center,
                direction: ray_direction,
            };

            *pixel = ray.color(world).into();
        }

        image
    }
}
