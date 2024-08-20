use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
use std::f64::consts::PI;

pub struct HitRecord<'m> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'m dyn Material,
    pub u: f64,
    pub v: f64,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = ray_t.max;

        for thing in self.iter() {
            if let Some(temp) = thing.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp.t;
                hit = Some(temp);
            }
        }

        hit
    }

    fn bounding_box(&self) -> Aabb {
        if self.is_empty() {
            return Aabb::from_points(Vec3::scalar(0.0), Vec3::scalar(0.0));
        }

        let mut bounds = Aabb::from_bounds(
            self.first().unwrap().bounding_box(),
            self.first().unwrap().bounding_box(),
        );
        for thing in self.iter() {
            bounds = Aabb::from_bounds(bounds, thing.bounding_box());
        }

        bounds
    }
}

pub enum Center {
    Stationary(Vec3),
    InMotion(Vec3, Vec3),
}

pub struct Sphere<M> {
    center: Center,
    radius: f64,
    material: M,
    bounds: Aabb,
}

impl<M> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Sphere<M> {
        let rvec = Vec3::scalar(radius);
        Sphere {
            center: Center::Stationary(center),
            radius,
            material,
            bounds: Aabb::from_points(center - rvec, center + rvec),
        }
    }

    pub fn moving(start: Vec3, end: Vec3, radius: f64, material: M) -> Sphere<M> {
        let rvec = Vec3::scalar(radius);
        let box1 = Aabb::from_points(start - rvec, start + rvec);
        let box2 = Aabb::from_points(end - rvec, end + rvec);

        Sphere {
            center: Center::InMotion(start, end - start),
            radius,
            material,
            bounds: Aabb::from_bounds(box1, box2),
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        match self.center {
            Center::Stationary(center) => center,
            Center::InMotion(start, direction) => start + direction * time,
        }
    }

    pub fn get_sphere_uv(&self, point: Vec3) -> (f64, f64) {
        let theta = (-point[1]).acos();
        let phi = (-point[2]).atan2(point[0]) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl<M> Hittable for Sphere<M>
where
    M: Material,
{
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
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

        let (u, v) = self.get_sphere_uv(outward_normal);

        Some(HitRecord {
            point,
            normal,
            t: root,
            front_face,
            material: &self.material,
            u,
            v,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bounds.clone()
    }
}

pub struct Quad<M> {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: M,
    bounds: Aabb,
    normal: Vec3,
    d: f64,
}

impl<M> Quad<M> {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: M) -> Quad<M> {
        let diagonal_1 = Aabb::from_points(q, q + u + v);
        let diagonal_2 = Aabb::from_points(q + u, q + v);
        let bounds = Aabb::from_bounds(diagonal_1, diagonal_2);

        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(q);

        let w = n / n.dot(n);

        Quad {
            q,
            u,
            v,
            w,
            material,
            bounds,
            normal,
            d,
        }
    }

    pub fn is_interior(a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        unit_interval.contains(a) && unit_interval.contains(b)
    }
}

impl<M> Hittable for Quad<M>
where
    M: Material,
{
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit = intersection - self.q;
        let alpha = self.w.dot(planar_hit.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit));
        let (u, v) = if Self::is_interior(alpha, beta) {
            (alpha, beta)
        } else {
            return None;
        };

        let front_face = ray.direction.dot(self.normal) < 0.0;
        let normal = if front_face {
            self.normal
        } else {
            -self.normal
        };

        Some(HitRecord {
            point: intersection,
            normal,
            t,
            front_face: false,
            material: &self.material,
            u,
            v,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bounds.clone()
    }
}

pub fn make_box(
    a: Vec3,
    b: Vec3,
    material: impl Material + Clone + 'static,
) -> Vec<Box<dyn Hittable>> {
    let min = Vec3([a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]);
    let max = Vec3([a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]);

    let dx = Vec3([max[0] - min[0], 0.0, 0.0]);
    let dy = Vec3([0.0, max[1] - min[1], 0.0]);
    let dz = Vec3([0.0, 0.0, max[2] - min[2]]);

    vec![
        Box::new(Quad::new(
            Vec3([min[0], min[1], max[2]]),
            dx,
            dy,
            material.clone(),
        )),
        Box::new(Quad::new(
            Vec3([max[0], min[1], max[2]]),
            -dz,
            dy,
            material.clone(),
        )),
        Box::new(Quad::new(
            Vec3([max[0], min[1], min[2]]),
            -dx,
            dy,
            material.clone(),
        )),
        Box::new(Quad::new(
            Vec3([min[0], min[1], min[2]]),
            dz,
            dy,
            material.clone(),
        )),
        Box::new(Quad::new(
            Vec3([min[0], max[1], max[2]]),
            dx,
            -dz,
            material.clone(),
        )),
        Box::new(Quad::new(
            Vec3([min[0], min[1], min[2]]),
            dx,
            dz,
            material.clone(),
        )),
    ]
}

pub struct Translate<H> {
    object: H,
    offset: Vec3,
    bounds: Aabb,
}

impl<H> Translate<H>
where
    H: Hittable,
{
    pub fn new(object: H, offset: Vec3) -> Translate<H> {
        let bounds = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bounds,
        }
    }
}

impl<H> Hittable for Translate<H>
where
    H: Hittable,
{
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };

        self.object.hit(&offset_r, ray_t).map(|mut hit| {
            hit.point += self.offset;
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bounds.clone()
    }
}

pub struct RotateY<H> {
    object: H,
    sin_theta: f64,
    cos_theta: f64,
    bounds: Aabb,
}

impl<H> RotateY<H>
where
    H: Hittable,
{
    pub fn new(object: H, angle: f64) -> RotateY<H> {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let original_bounds = object.bounding_box();

        let mut min = Vec3::scalar(f64::MAX);
        let mut max = Vec3::scalar(f64::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x =
                        i as f64 * original_bounds.x.max + (1.0 - i as f64) * original_bounds.x.min;
                    let y =
                        j as f64 * original_bounds.y.max + (1.0 - j as f64) * original_bounds.y.min;
                    let z =
                        k as f64 * original_bounds.z.max + (1.0 - k as f64) * original_bounds.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3([newx, y, newz]);

                    for c in 0..3 {
                        min.0[c] = min[c].min(tester[c]);
                        max.0[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bounds = Aabb::from_points(min, max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bounds,
        }
    }
}

impl<H> Hittable for RotateY<H>
where
    H: Hittable,
{
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin.0[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin.0[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction.0[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction.0[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated = Ray {
            origin,
            direction,
            time: ray.time,
        };

        self.object.hit(&rotated, ray_t).map(|mut hit| {
            let mut p = hit.point;
            p.0[0] = self.cos_theta * hit.point[0] + self.sin_theta * hit.point[2];
            p.0[2] = -self.sin_theta * hit.point[0] + self.cos_theta * hit.point[2];

            let mut normal = hit.normal;
            normal.0[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
            normal.0[0] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];

            hit.point = p;
            hit.normal = normal;

            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bounds.clone()
    }
}

pub struct ConstantMedium<H, M> {
    boundary: H,
    neg_inv_density: f64,
    phase_function: M,
}

impl<H, M> ConstantMedium<H, M> {
    pub fn new(boundary: H, density: f64, material: M) -> ConstantMedium<H, M> {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: material,
        }
    }
}

impl<H, M> Hittable for ConstantMedium<H, M>
where
    H: Hittable,
    M: Material,
{
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if let Some(mut hit) = self.boundary.hit(ray, Interval::new(f64::MIN, f64::MAX)) {
            if let Some(mut hit2) = self
                .boundary
                .hit(ray, Interval::new(hit.t + 0.0001, f64::MAX))
            {
                let mut rand = rand::thread_rng();

                if hit.t < ray_t.min {
                    hit.t = ray_t.min;
                }

                if hit2.t > ray_t.max {
                    hit2.t = ray_t.max;
                }

                if hit.t >= hit2.t {
                    return None;
                }

                if hit.t < 0.0 {
                    hit.t = 0.0
                }

                let ray_length = ray.direction.length();
                let distance_inside_boundary = (hit2.t - hit.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand.gen::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                Some(HitRecord {
                    point: ray.at(hit.t),
                    normal: Vec3([1.0, 0.0, 0.0]),
                    t: hit.t + hit_distance / ray_length,
                    front_face: true,
                    material: &self.phase_function,
                    u: 0.0,
                    v: 0.0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box().clone()
    }
}
