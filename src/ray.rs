use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color<H: Hittable + ?Sized>(&self, depth: u32, world: &H) -> Vec3 {
        if depth == 0 {
            return Vec3::scalar(0.0);
        }

        if let Some(hit) = world.hit(self, Interval::new(0.001, f64::MAX)) {
            if let Some((scattered, attenuation)) = hit.material.scatter(self, &hit) {
                return attenuation * scattered.color(depth - 1, world);
            }
            return Vec3::scalar(0.0);
        }

        let unit_direction = self.direction.unit();
        let a = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - a) * Vec3::scalar(1.0) + a * Vec3([0.5, 0.7, 1.0])
    }
}
