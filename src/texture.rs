use image::RgbImage;
use nalgebra::{vector, SVector, Unit};
use rand::Rng;

use crate::NEG_ONE_ONE_CLOSED;

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, point: SVector<f64, 3>) -> SVector<f64, 3>;
}

#[derive(Debug, Clone)]
pub struct SolidColor {
    color: SVector<f64, 3>,
}

impl SolidColor {
    pub fn new(color: SVector<f64, 3>) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: SVector<f64, 3>) -> SVector<f64, 3> {
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
    fn value(&self, u: f64, v: f64, point: SVector<f64, 3>) -> SVector<f64, 3> {
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
    fn value(&self, u: f64, v: f64, _point: SVector<f64, 3>) -> SVector<f64, 3> {
        if self.image.height() == 0 {
            return vector![0.0, 1.0, 1.0];
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let pixel = self.image.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;

        vector![
            color_scale * pixel.0[0] as f64,
            color_scale * pixel.0[1] as f64,
            color_scale * pixel.0[2] as f64,
        ]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Perlin<const N: usize> {
    randvec: [Unit<SVector<f64, 3>>; N],
    perm_x: [i32; N],
    perm_y: [i32; N],
    perm_z: [i32; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Perlin<N> {
        let mut rand = rand::thread_rng();
        let mut vecs = [Unit::new_normalize(SVector::<f64, 3>::zeros()); N];
        for p in vecs.iter_mut() {
            *p = Unit::new_normalize(SVector::<f64, 3>::from_distribution(&*NEG_ONE_ONE_CLOSED, &mut rand));
        }

        Perlin {
            randvec: vecs,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    fn generate_perm() -> [i32; N] {
        let mut perm = [0; N];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = i as i32;
        }

        Perlin::permute(perm)
    }

    fn permute(mut perm: [i32; N]) -> [i32; N] {
        let mut rand = rand::thread_rng();
        for i in (1..N).rev() {
            let target = rand.gen_range(0..i);
            perm.swap(i, target);
        }

        perm
    }

    pub fn noise(&self, point: SVector<f64, 3>) -> f64 {
        let u = point[0] - point[0].floor();
        let v = point[1] - point[1].floor();
        let w = point[2] - point[2].floor();

        let i = point[0].floor() as i32;
        let j = point[1].floor() as i32;
        let k = point[2].floor() as i32;

        let mut c = [[[Unit::new_normalize(SVector::<f64, 3>::zeros()); 2]; 2]; 2];
        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.randvec[(self.perm_x[((i + di as i32) & (N - 1) as i32) as usize]
                        ^ self.perm_y[((j + dj as i32) & (N - 1) as i32) as usize]
                        ^ self.perm_z[((k + dk as i32) & (N - 1) as i32) as usize])
                        as usize]
                }
            }
        }

        Perlin::<N>::perlin_interpolation(c, u, v, w)
    }

    pub fn perlin_interpolation(c: [[[Unit<SVector<f64, 3>>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cj) in ci.iter().enumerate() {
                for (k, ck) in cj.iter().enumerate() {
                    let weight = vector![u - i as f64, v - j as f64, w - k as f64];
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * ck.dot(&weight);
                }
            }
        }

        accum
    }

    pub fn turbulence(&self, point: SVector<f64, 3>, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Noise<const N: usize> {
    noise: Perlin<N>,
    scale: f64,
}

impl<const N: usize> Noise<N> {
    pub fn new(scale: f64) -> Noise<N> {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl<const N: usize> Texture for Noise<N> {
    fn value(&self, _u: f64, _v: f64, point: SVector<f64, 3>) -> SVector<f64, 3> {
        SVector::<f64, 3>::repeat(0.5)
            * (1.0 + (self.scale * point[2] + 10.0 * self.noise.turbulence(point, 7)).sin())
    }
}
