use image::Rgb;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(lhs, rhs)| lhs * rhs)
            .sum()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
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

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
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

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
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
        Rgb([
            (value[0].clamp(0.0, 0.999) * 256.0) as u8,
            (value[1].clamp(0.0, 0.999) * 256.0) as u8,
            (value[2].clamp(0.0, 0.999) * 256.0) as u8,
        ])
    }
}
