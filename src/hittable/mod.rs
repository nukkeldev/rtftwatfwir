use glam::DVec3;

use crate::{
    bvh::aabb::AABB,
    material::Material,
    util::{interval::Interval, ray::Ray, Point3},
};

pub mod hittable_list;
pub mod sphere;

pub struct HitRecord<'mat> {
    /// Absolute point of the hit
    pub p: Point3,
    /// Direction of the normal
    pub normal: DVec3,
    /// What material was hit?
    pub material: &'mat Material,
    /// Length into ray; P(t) = A + tB
    pub t: f64,
    /// UV Coordinates of the texture
    pub u: f64,
    pub v: f64,
    /// Whether or not the normal is an outward facing face.
    pub front_face: bool,
}

impl<'mat> HitRecord<'mat> {
    pub fn new(
        r: &Ray,
        t: f64,
        p: Point3,
        u: f64,
        v: f64,
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
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit<'mat>(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
}
