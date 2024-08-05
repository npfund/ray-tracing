use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Range;

pub struct HitRecord<'m> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'m dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}

impl Hittable for &[Box<dyn Hittable>] {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = ray_t.end;

        for thing in self.iter() {
            if let Some(temp) = thing.hit(ray, ray_t.start..closest_so_far) {
                closest_so_far = temp.t;
                hit = Some(temp);
            }
        }

        hit
    }
}

pub enum Center {
    Stationary(Vec3),
    InMotion(Vec3, Vec3),
}

pub struct Sphere {
    center: Center,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center: Center::Stationary(center),
            radius,
            material,
        }
    }

    pub fn moving(start: Vec3, end: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center: Center::InMotion(start, end - start),
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        match self.center {
            Center::Stationary(center) => center,
            Center::InMotion(start, direction) => start + direction * time,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let center = self.center(ray.time);
        let oc = center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_t.start || ray_t.end <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.start || ray_t.end <= root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - center) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord {
            point,
            normal,
            t: root,
            front_face,
            material: &*self.material,
        })
    }
}
