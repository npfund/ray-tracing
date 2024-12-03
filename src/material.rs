use nalgebra::{SVector, Unit};
use rand::Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::{near_zero, random_unit_vector, reflect, refract};

pub trait Material: Sync {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, SVector<f64, 3>)> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _point: SVector<f64, 3>) -> SVector<f64, 3> {
        SVector::<f64, 3>::zeros()
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian<T> {
    pub texture: T,
}

impl<T> Material for Lambertian<T>
where
    T: Texture,
{
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, SVector<f64, 3>)> {
        let potential_direction = *hit.normal + *random_unit_vector();
        let direction = if near_zero(potential_direction) {
            hit.normal
        } else {
            Unit::new_normalize(potential_direction)
        };

        let scattered = Ray {
            origin: hit.point,
            direction,
            time: ray.time,
        };

        Some((scattered, self.texture.value(hit.u, hit.v, hit.point)))
    }
}

pub struct Metal {
    pub albedo: SVector<f64, 3>,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, SVector<f64, 3>)> {
        let reflected = *Unit::new_normalize(reflect(*ray.direction, *hit.normal))
            + (self.fuzz * *random_unit_vector());
        let scattered = Ray {
            origin: hit.point,
            direction: Unit::new_normalize(reflected),
            time: ray.time,
        };

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, SVector<f64, 3>)> {
        let attenuation = SVector::<f64, 3>::repeat(1.0);
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction;
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let mut rand = rand::thread_rng();
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand.gen::<f64>() {
                reflect(*unit_direction, *hit.normal)
            } else {
                refract(*unit_direction, *hit.normal, ri)
            };

        let scattered = Ray {
            origin: hit.point,
            direction: Unit::new_normalize(direction),
            time: ray.time,
        };

        Some((scattered, attenuation))
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight<T> {
    texture: T,
}

impl<T> DiffuseLight<T>
where
    T: Texture,
{
    pub fn new(texture: T) -> DiffuseLight<T> {
        DiffuseLight { texture }
    }
}

impl<T> Material for DiffuseLight<T>
where
    T: Texture,
{
    fn emitted(&self, u: f64, v: f64, point: SVector<f64, 3>) -> SVector<f64, 3> {
        self.texture.value(u, v, point)
    }
}

pub struct Isotropic<T> {
    texture: T,
}

impl<T> Isotropic<T> {
    pub fn new(texture: T) -> Isotropic<T> {
        Isotropic { texture }
    }
}

impl<T> Material for Isotropic<T>
where
    T: Texture,
{
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, SVector<f64, 3>)> {
        Some((
            Ray {
                origin: hit.point,
                direction: random_unit_vector(),
                time: ray.time,
            },
            self.texture.value(hit.u, hit.v, hit.point),
        ))
    }
}
