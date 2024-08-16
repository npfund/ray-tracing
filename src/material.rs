use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use rand::Rng;

pub trait Material: Sync {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        Vec3::scalar(0.0)
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
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let potential_direction = hit.normal + Vec3::random_unit_vector();
        let direction = if potential_direction.near_zero() {
            hit.normal
        } else {
            potential_direction
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
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(ray.direction, hit.normal).unit()
            + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray {
            origin: hit.point,
            direction: reflected,
            time: ray.time,
        };

        if scattered.direction.dot(hit.normal) > 0.0 {
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
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::scalar(1.0);
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let mut rand = rand::thread_rng();
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand.gen::<f64>() {
                Vec3::reflect(unit_direction, hit.normal)
            } else {
                Vec3::refract(unit_direction, hit.normal, ri)
            };

        let scattered = Ray {
            origin: hit.point,
            direction,
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
    fn emitted(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        self.texture.value(u, v, point)
    }
}
