use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    Point3, aabb::AABB,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    origin: Point3,
    radius: f64,
    material: Material,

    is_moving: bool,
    movement_vec: DVec3,

    bbox: AABB
}

impl Sphere {
    pub fn new_stationary(origin: Point3, radius: f64, material: Material) -> Self {
        let rvec = DVec3::new(radius, radius, radius);
        Self {
            origin,
            radius,
            material,
            is_moving: false,
            movement_vec: DVec3::ZERO,
            bbox: AABB::from_points(origin - rvec, origin + rvec)
        }
    }

    pub fn new_moving(origin: Point3, endpoint: Point3, radius: f64, material: Material) -> Self {
        let rvec = DVec3::new(radius, radius, radius);
        let box_1 = AABB::from_points(origin - rvec, origin + rvec);
        let box_2 = AABB::from_points(endpoint - rvec, endpoint + rvec);
        let bbox = AABB::from_boxes(&box_1, &box_2);

        Self {
            origin,
            radius,
            material,
            is_moving: true,
            movement_vec: endpoint - origin,
            bbox
        }
    }

    pub fn position(&self, time: f64) -> Point3 {
        self.origin + time * self.movement_vec
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
        Some(HitRecord::new(r, root, p, &self.material, outward_normal))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
