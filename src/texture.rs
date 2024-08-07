use crate::vec3::Vec3;
use image::RgbImage;

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}

#[derive(Debug, Clone)]
pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct Checker<E, O> {
    pub inv_scale: f64,
    pub even: E,
    pub odd: O,
}

impl<E, O> Checker<E, O> {
    pub fn new(scale: f64, even: E, odd: O) -> Checker<E, O> {
        Checker {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl<E, O> Texture for Checker<E, O>
where
    E: Texture,
    O: Texture,
{
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        let x = (self.inv_scale * point[0]).floor();
        let y = (self.inv_scale * point[1]).floor();
        let z = (self.inv_scale * point[2]).floor();

        if (x + y + z) % 2.0 == 0.0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(image: RgbImage) -> Image {
        Image { image }
    }
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, _point: Vec3) -> Vec3 {
        if self.image.height() == 0 {
            return Vec3([0.0, 1.0, 1.0]);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let pixel = self.image.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;

        Vec3([
            color_scale * pixel.0[0] as f64,
            color_scale * pixel.0[1] as f64,
            color_scale * pixel.0[2] as f64,
        ])
    }
}
