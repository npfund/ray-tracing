use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;

pub struct Node {
    left: Box<dyn Hittable>,
    right: Option<Box<dyn Hittable>>,
    bounds: Aabb,
}

impl Node {
    pub fn from_list(mut objects: Vec<Box<dyn Hittable>>) -> Node {
        let mut bounds = Aabb::new(Interval::empty(), Interval::empty(), Interval::empty());
        for object in &objects {
            bounds = Aabb::from_bounds(bounds, object.bounding_box())
        }

        let (left, right) = if objects.len() == 1 {
            (objects.pop().unwrap(), None)
        } else if objects.len() == 2 {
            let right = objects.pop().unwrap();
            let left = objects.pop().unwrap();
            (left, Some(right))
        } else {
            let axis = bounds.longest_axis();

            objects.sort_unstable_by(|l, r| match axis {
                0 => Self::box_compare(l.as_ref(), r.as_ref(), 0),
                1 => Self::box_compare(l.as_ref(), r.as_ref(), 1),
                _ => Self::box_compare(l.as_ref(), r.as_ref(), 2),
            });

            let mid = objects.len() / 2;
            let left: Box<dyn Hittable> =
                Box::new(Node::from_list(objects.drain(0..mid).collect()));
            let right: Box<dyn Hittable> = Box::new(Node::from_list(objects));

            (left, Some(right))
        };

        Node {
            left,
            right,
            bounds,
        }
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: u32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);

        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }
}

impl Hittable for Node {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bounds.hit(ray, ray_t) {
            return None;
        }

        if let Some(hit_left) = self.left.hit(ray, ray_t) {
            if let Some(right) = &self.right {
                right
                    .hit(ray, Interval::new(ray_t.min, hit_left.t))
                    .or(Some(hit_left))
            } else {
                Some(hit_left)
            }
        } else if let Some(right) = &self.right {
            right.hit(ray, ray_t)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounds.clone()
    }
}
