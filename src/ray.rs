use nalgebra::{SVector, Unit};

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::vec3;

pub struct Ray {
    pub origin: SVector<f64, 3>,
    pub direction: Unit<SVector<f64, 3>>,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> SVector<f64, 3> {
        self.origin + *self.direction * t
    }

    pub fn color<H: Hittable + ?Sized>(&self, depth: u32, world: &H, background: SVector<f64, 3>) -> SVector<f64, 3> {
        if depth == 0 {
            return SVector::<f64, 3>::zeros();
        }

        if let Some(hit) = world.hit(self, Interval::new(0.001, f64::MAX)) {
            let emission = hit.material.emitted(hit.u, hit.v, hit.point);
            if let Some((scattered, attenuation)) = hit.material.scatter(self, &hit) {
                return emission + vec3::mul(attenuation, scattered.color(depth - 1, world, background));
            }
            return emission;
        }

        background
    }
}
