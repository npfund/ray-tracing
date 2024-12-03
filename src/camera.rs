use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{vector, SVector, Unit};
use rand::Rng;
use rayon::prelude::*;

use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: SVector<f64, 3>,
    pixel00_loc: SVector<f64, 3>,
    pixel_delta_u: SVector<f64, 3>,
    pixel_delta_v: SVector<f64, 3>,
    samples_per_pixel: u32,
    max_depth: u32,
    defocus_angle: f64,
    defocus_disk_u: SVector<f64, 3>,
    defocus_disk_v: SVector<f64, 3>,
    background: SVector<f64, 3>,
}

impl Camera {
    //todo builder?
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: SVector<f64, 3>,
        look_at: SVector<f64, 3>,
        vup: SVector<f64, 3>,
        defocus_angle: f64,
        focus_dist: f64,
        background: SVector<f64, 3>,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let center = look_from;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = Unit::new_normalize(look_from - look_at);
        let u = Unit::new_normalize(vup.cross(&w));
        let v = w.cross(&u);

        let viewport_u = viewport_width * *u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * *w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = *u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background,
        }
    }

    pub fn render<H: Hittable + ?Sized>(&self, world: &H) -> RgbImage {
        let mut image = RgbImage::new(self.image_width, self.image_height);
        let total = self.image_width * self.image_height;

        let progress_bar = ProgressBar::new(total as u64).with_style(
            ProgressStyle::with_template("{wide_bar} {pos}/{len} {elapsed_precise}").unwrap(),
        );
        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let mut color = SVector::<f64, 3>::zeros();
            for _ in 0..self.samples_per_pixel {
                let temp = self
                    .get_ray(x, y)
                    .color(self.max_depth, world, self.background);
                color += temp;
            }

            let avg = color / self.samples_per_pixel as f64;
            let mut smudged = avg.into_iter().map(|&x| {
                if x > 0.0 {
                    (x.sqrt().clamp(0.0, 0.999) * 256.0) as u8
                } else {
                    0
                }
            });

            *pixel = Rgb([
                smudged.next().unwrap(),
                smudged.next().unwrap(),
                smudged.next().unwrap(),
            ]);

            progress_bar.inc(1);
        });

        progress_bar.finish();

        image
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (x as f64 + offset[0]) * self.pixel_delta_u
            + (y as f64 + offset[1]) * self.pixel_delta_v;
        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let direction = Unit::new_normalize(pixel_sample - origin);

        let mut rand = rand::thread_rng();
        Ray {
            origin,
            direction,
            time: rand.gen(),
        }
    }

    fn sample_square() -> SVector<f64, 3> {
        let mut rng = rand::thread_rng();
        vector![rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0]
    }

    fn defocus_disk_sample(&self) -> SVector<f64, 3> {
        let mut rand = rand::thread_rng();
        let p = loop {
            let p = vector![rand.gen_range(-1.0..1.0), rand.gen_range(-1.0..1.0), 0.0];
            if p.magnitude_squared() < 1.0 {
                break p
            }
        };

        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}
