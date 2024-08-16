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

    pub fn color<H: Hittable + ?Sized>(&self, depth: u32, world: &H, background: Vec3) -> Vec3 {
        if depth == 0 {
            return Vec3::scalar(0.0);
        }

        if let Some(hit) = world.hit(self, Interval::new(0.001, f64::MAX)) {
            let emission = hit.material.emitted(hit.u, hit.v, hit.point);
            if let Some((scattered, attenuation)) = hit.material.scatter(self, &hit) {
                return emission + attenuation * scattered.color(depth - 1, world, background);
            }
            return emission;
        }

        background
    }
}
