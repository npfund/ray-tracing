use image::Rgb;
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn scalar(s: f64) -> Vec3 {
        Vec3([s, s, s])
    }

    pub fn x(x: f64) -> Vec3 {
        Vec3([x, 0.0, 0.0])
    }

    pub fn y(y: f64) -> Vec3 {
        Vec3([0.0, y, 0.0])
    }

    pub fn z(z: f64) -> Vec3 {
        Vec3([0.0, 0.0, z])
    }

    pub fn random() -> Vec3 {
        let mut rand = rand::thread_rng();
        Vec3([rand.gen::<f64>(), rand.gen::<f64>(), rand.gen::<f64>()])
    }

    pub fn random_within(min: f64, max: f64) -> Vec3 {
        let mut rand = rand::thread_rng();
        Vec3([
            rand.gen_range(min..max),
            rand.gen_range(min..max),
            rand.gen_range(min..max),
        ])
    }

    fn random_within_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_within(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_within_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_sphere = Vec3::random_unit_vector();
        if on_sphere.dot(normal) > 0.0 {
            on_sphere
        } else {
            -on_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rand = rand::thread_rng();
        loop {
            let p = Vec3([rand.gen_range(-1.0..1.0), rand.gen_range(-1.0..1.0), 0.0]);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn length_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(lhs, rhs)| lhs * rhs)
            .sum()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(n).min(1.0);
        let perp = etai_over_etat * (uv + cos_theta * n);
        let parallel = -(1.0 - perp.length_squared()).abs().sqrt() * n;

        perp + parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3([-self[0], -self[1], -self[2]])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(lhs, rhs)| *lhs += rhs);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3([rhs[0] * self, rhs[1] * self, rhs[2] * self])
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl From<Vec3> for Rgb<u8> {
    fn from(value: Vec3) -> Self {
        let mut smudged = value.0.iter().map(|&x| {
            if x > 0.0 {
                (x.sqrt().clamp(0.0, 0.999) * 256.0) as u8
            } else {
                0
            }
        });

        Rgb([
            smudged.next().unwrap(),
            smudged.next().unwrap(),
            smudged.next().unwrap(),
        ])
    }
}
