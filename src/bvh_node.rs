use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    random_int_range,
    ray::Ray,
};

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: AABB,
}

impl BVHNode {
    pub fn from_list(hittable_list: &HittableList) -> Self {
        Self::new(&hittable_list.objects[..])
    }

    fn new(objects: &[Arc<dyn Hittable>]) -> Self {
        let axis = random_int_range(0, 2) as usize;

        match objects.len() {
            1 => {
                let obj = objects[0].clone();
                Self {
                    bbox: obj.bounding_box().clone(),
                    left: obj.clone(),
                    right: obj.clone(),
                }
            }
            2 => {
                let (left, right) = if Self::box_compare(&objects[0], &objects[1], axis) {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                };

                let bbox = AABB::from_boxes(left.bounding_box(), right.bounding_box());

                Self { left, right, bbox }
            }
            _ => {
                let mut objects = objects.to_vec();
                objects.sort_by(move |a, b| {
                    if Self::box_compare(a, b, axis) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                let mid = objects.len() / 2;
                let left = Arc::new(BVHNode::new(&objects[0..mid]));
                let right = Arc::new(BVHNode::new(&objects[mid..objects.len()]));
                let bbox = AABB::from_boxes(left.bounding_box(), right.bounding_box());

                Self { left, right, bbox }
            }
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> bool {
        a.bounding_box().axis(axis).min < b.bounding_box().axis(axis).min
    }
}

impl Hittable for BVHNode {
    fn hit<'mat>(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if self.bbox.hit(r, ray_t).is_none() {
            return None;
        }

        match self.left.hit(r, ray_t) {
            Some(left_hit) => Some(
                self.right
                    .hit(r, Interval::new(ray_t.min, left_hit.t))
                    .unwrap_or(left_hit),
            ),
            None => self.right.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
