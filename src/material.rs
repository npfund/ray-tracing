use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let potential_direction = hit.normal + Vec3::random_unit_vector();
        let direction = if potential_direction.near_zero() {
            hit.normal
        } else {
            potential_direction
        };

        let scattered = Ray {
            origin: hit.point,
            direction,
        };

        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&ray.direction, &hit.normal).unit()
            + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray {
            origin: hit.point,
            direction: reflected,
        };

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
