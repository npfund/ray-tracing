use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub fn from_bounds(a: Aabb, b: Aabb) -> Aabb {
        Aabb {
            x: Interval::merge(a.x, b.x),
            y: Interval::merge(a.y, b.y),
            z: Interval::merge(a.z, b.z),
        }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Aabb {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };

        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };

        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        Aabb { x, y, z }
    }

    pub fn axis_interval(&self, n: u32) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn longest_axis(&self) -> u32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        for axis in 0..3 {
            let interval = self.axis_interval(axis);
            let adinv = 1.0 / ray.direction[axis as usize];

            let t0 = (interval.min - ray.origin[axis as usize]) * adinv;
            let t1 = (interval.max - ray.origin[axis as usize]) * adinv;

            let mut min = ray_t.min;
            let mut max = ray_t.max;
            if t0 < t1 {
                if t0 > min {
                    min = t0
                }
                if t1 < max {
                    max = t1
                }
            } else {
                if t1 > min {
                    min = t1
                }
                if t0 < max {
                    max = t0
                }
            }

            if max <= min {
                return false;
            }
        }

        true
    }
}
