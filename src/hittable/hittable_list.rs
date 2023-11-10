use std::sync::Arc;

use crate::{
    bvh::aabb::AABB,
    util::{interval::Interval, ray::Ray},
};

use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: AABB::new_empty(),
        }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.bbox.expand(object.bounding_box());
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        let mut f_rec = None;
        let mut closest = ray_t.max;

        for object in self.objects.iter() {
            ray_t.max = closest;
            if let Some(rec) = object.hit(r, ray_t) {
                closest = rec.t;
                f_rec = Some(rec);
            }
        }

        f_rec
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
