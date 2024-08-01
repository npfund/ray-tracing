use crate::hittable::Hittable;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, world: &[Box<dyn Hittable>]) -> Vec3 {
        if let Some(hit) = world.hit(self, 0.0..f64::MAX) {
            return 0.5 * (hit.normal + Vec3::scalar(1.0));
        }

        let unit_direction = self.direction.unit();
        let a = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - a) * Vec3::scalar(1.0) + a * Vec3([0.5, 0.7, 1.0])
    }
}
