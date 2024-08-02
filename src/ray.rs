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

    pub fn color(&self, depth: u32, world: &[Box<dyn Hittable>]) -> Vec3 {
        if depth == 0 {
            return Vec3::scalar(0.0);
        }

        if let Some(hit) = world.hit(self, 0.001..f64::MAX) {
            let direction = hit.normal + Vec3::random_unit_vector();
            let ray = Ray { origin: hit.point, direction };
            return 0.5 * ray.color(depth - 1, world);
        }

        let unit_direction = self.direction.unit();
        let a = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - a) * Vec3::scalar(1.0) + a * Vec3([0.5, 0.7, 1.0])
    }
}
