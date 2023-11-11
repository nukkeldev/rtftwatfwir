use glam::Vec3A;

use crate::{bvh::aabb::AABB, material::Material, util::Point3};

use super::{HitRecord, Hittable};
use crate::util::all::*;

pub struct Quad {
    q: Point3,
    u: Vec3A,
    v: Vec3A,
    mat: Material,
    bbox: AABB,

    normal: Vec3A,
    d: f32,
    w: Vec3A,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3A, v: Vec3A, mat: Material) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();

        let mut s = Self {
            q,
            u,
            v,
            mat,
            bbox: AABB::new_empty(),
            normal,
            d: normal.dot(q),
            w: n / n.dot(n),
        };
        s.update_bounding_box();
        s
    }

    pub fn update_bounding_box(&mut self) {
        self.bbox = AABB::from_points(self.q, self.q + self.u + self.v).pad();
    }

    fn is_interior(&self, a: f32, b: f32) -> bool {
        (a >= 0.0) && (a <= 1.0) && (b >= 0.0) && (b <= 1.0)
    }
}

impl Hittable for Quad {
    fn hit<'mat>(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);

        // r is parallel
        if denom.abs() < 1.0e-8 {
            return None;
        }

        // Return None if the hit point parameter t is outside the interval.
        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hit_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        if !self.is_interior(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            r,
            t,
            intersection,
            alpha,
            beta,
            &self.mat,
            self.normal,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
