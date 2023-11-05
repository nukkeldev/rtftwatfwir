use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, mut t_bounds: Interval<f64>) -> Option<HitRecord> {
        let mut f_rec = None;
        let mut closest = t_bounds.max;

        for object in self.objects.iter() {
            t_bounds.max = closest;
            if let Some(rec) = object.hit(r, t_bounds) {
                closest = rec.t;
                f_rec = Some(rec);
            }
        }

        f_rec
    }
}
