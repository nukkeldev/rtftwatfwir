use std::f32::consts::PI;

use glam::Vec3A;

use crate::hittable::Hittable;

use super::{onb::ONB, random::random_unit_vector, vec::random_cosine_direction, Point3};

pub trait PDF {
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}

pub struct SpherePDF;

impl PDF for SpherePDF {
    fn value(&self, _: Vec3A) -> f32 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3A {
        random_unit_vector()
    }
}

pub struct CosinePDF {
    pub uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3A) -> Self {
        Self {
            uvw: ONB::new_from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        let cosine_theta = direction.normalize().dot(self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3A {
        self.uvw.local(random_cosine_direction())
    }
}

// Hittable PDF
pub struct HittablePDF<'a> {
    pub objects: &'a dyn Hittable,
    pub origin: Point3,
}

impl<'a> HittablePDF<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl<'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: Vec3A) -> f32 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3A {
        self.objects.random(self.origin)
    }
}
