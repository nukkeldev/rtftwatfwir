use std::f32::consts::PI;

use glam::Vec3A;

use crate::{
    bvh::aabb::AABB,
    material::Material,
    util::{all::Ray, interval::Interval, Point3},
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    origin: Point3,
    radius: f32,
    material: Material,

    is_moving: bool,
    movement_vec: Vec3A,

    bbox: AABB,
}

impl Sphere {
    pub fn new_stationary(origin: Point3, radius: f32, material: Material) -> Self {
        let rvec = Vec3A::new(radius, radius, radius);
        Self {
            origin,
            radius,
            material,
            is_moving: false,
            movement_vec: Vec3A::ZERO,
            bbox: AABB::from_points(origin - rvec, origin + rvec),
        }
    }

    pub fn new_moving(origin: Point3, endpoint: Point3, radius: f32, material: Material) -> Self {
        let rvec = Vec3A::new(radius, radius, radius);
        let box_1 = AABB::from_points(origin - rvec, origin + rvec);
        let box_2 = AABB::from_points(endpoint - rvec, endpoint + rvec);
        let bbox = AABB::from_boxes(&box_1, &box_2);

        Self {
            origin,
            radius,
            material,
            is_moving: true,
            movement_vec: endpoint - origin,
            bbox,
        }
    }

    pub fn position(&self, time: f32) -> Point3 {
        self.origin + time * self.movement_vec
    }

    fn get_sphere_uv(p: Point3) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit<'mat>(&'mat self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let position = if self.is_moving {
            self.position(r.time)
        } else {
            self.origin
        };

        let oc = r.origin - position;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrt_d) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.origin) / self.radius;
        let (u, v) = Self::get_sphere_uv(outward_normal);
        Some(HitRecord::new(
            r,
            root,
            p,
            u,
            v,
            &self.material,
            outward_normal,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
