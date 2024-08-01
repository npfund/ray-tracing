use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Range;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
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
        let outward_normal = (point - self.center) / self.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
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
        })
    }
}
