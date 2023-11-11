use glam::Vec3A;

use crate::{
    bvh::aabb::AABB,
    material::Material,
    util::{interval::Interval, ray::Ray, Point3},
};

use self::{hittable_list::HittableList, quad::Quad};

pub mod constant_medium;
pub mod hittable_list;
pub mod instance;
pub mod quad;
pub mod sphere;

pub struct HitRecord<'mat> {
    /// Absolute point of the hit
    pub p: Point3,
    /// Direction of the normal
    pub normal: Vec3A,
    /// What material was hit?
    pub material: &'mat Material,
    /// Length into ray; P(t) = A + tB
    pub t: f32,
    /// UV Coordinates of the texture
    pub u: f32,
    pub v: f32,
    /// Whether or not the normal is an outward facing face.
    pub front_face: bool,
}

impl<'mat> HitRecord<'mat> {
    pub fn new(
        r: &Ray,
        t: f32,
        p: Point3,
        u: f32,
        v: f32,
        material: &'mat Material,
        outward_normal: Vec3A,
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

/// Returns a 3D box that contains the two opposite verticies, a and b.
pub fn new_box(a: Point3, b: Point3, mat: Material) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3A::X * (max.x - min.x);
    let dy = Vec3A::Y * (max.y - min.y);
    let dz = Vec3A::Z * (max.z - min.z);

    sides.add(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )); // Front
    sides.add(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )); // Right
    sides.add(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )); // Back
    sides.add(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )); // Left
    sides.add(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )); // Top
    sides.add(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    )); // Bottom

    sides
}
