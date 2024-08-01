use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
