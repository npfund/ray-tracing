use image::{ImageBuffer, Rgb};
use crate::ray::Ray;
use crate::vec3::Vec3;

mod ray;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 256;
    let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3([0.0, 0.0, 0.0]);

    let viewport_u = Vec3([viewport_width, 0.0, 0.0]);
    let viewport_v = Vec3([0.0, -viewport_height, 0.0]);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3([0.0, 0.0, focal_length]) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let pixel_center = pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
        let ray_direction = pixel_center - camera_center;
        let ray = Ray {
            origin: camera_center,
            direction: ray_direction
        };

        let color: Rgb<u8> = ray.color().into();
        *pixel = color;
    }

    image.save("temp.png").unwrap();
}
