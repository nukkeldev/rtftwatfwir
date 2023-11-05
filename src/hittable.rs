use glam::DVec3;

use crate::{interval::Interval, material::Material, ray::Ray, Point3};

#[derive(Debug)]
pub struct HitRecord<'mat> {
    pub p: Point3,
    pub normal: DVec3,
    pub material: &'mat Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'mat> HitRecord<'mat> {
    pub fn new(
        r: &Ray,
        t: f64,
        p: Point3,
        material: &'mat Material,
        outward_normal: DVec3,
    ) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        Self {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit<'mat>(&self, r: &Ray, t_bounds: Interval<f64>) -> Option<HitRecord>;
}
