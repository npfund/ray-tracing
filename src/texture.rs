use crate::vec3::Vec3;

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}

pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> SolidColor {
        SolidColor { albedo: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        self.albedo
    }
}

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
