use crate::vec3::Vec3;

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub albedo: Vec3,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct Checker {
    pub inv_scale: f64,
    pub even: Box<dyn Texture>,
    pub odd: Box<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Vec3, odd: Vec3) -> Checker {
        let even = SolidColor { albedo: even };
        let odd = SolidColor { albedo: odd };

        Checker {
            inv_scale: 1.0 / scale,
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }
}

impl Texture for Checker {
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
