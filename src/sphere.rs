use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    Point3,
};

#[derive(Debug)]
pub struct Sphere {
    origin: Point3,
    radius: f64,
    material: Material,

    is_moving: bool,
    movement_vec: DVec3,
}

impl Sphere {
    pub fn new_stationary(origin: Point3, radius: f64, material: Material) -> Self {
        Self {
            origin,
            radius,
            material,
            is_moving: false,
            movement_vec: DVec3::ZERO
        }
    }

    pub fn new_moving(origin: Point3, endpoint: Point3, radius: f64, material: Material) -> Self {
        Self {
            origin,
            radius,
            material,
            is_moving: true,
            movement_vec: endpoint - origin
        }
    }

    pub fn position(&self, time: f64) -> Point3 {
        self.origin + time * self.movement_vec
    }
}

impl Hittable for Sphere {
    fn hit<'mat>(&'mat self, r: &Ray, t_bounds: Interval<f64>) -> Option<HitRecord> {
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
        if !t_bounds.contains(root) {
            root = (-half_b + sqrt_d) / a;
            if !t_bounds.contains(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.origin) / self.radius;
        Some(HitRecord::new(r, root, p, &self.material, outward_normal))
    }
}
