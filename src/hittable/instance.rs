use std::sync::Arc;

use glam::DVec3;

use crate::bvh::aabb::AABB;

use super::{HitRecord, Hittable};
use crate::util::all::*;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: DVec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: impl Hittable + 'static, offset: DVec3) -> Self {
        Self {
            bbox: object.bounding_box().clone() + offset,
            object: Arc::new(object),
            offset,
        }
    }
}

impl Hittable for Translate {
    fn hit<'mat>(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new_with_time(r.origin - self.offset, r.direction, r.time);

        self.object.hit(&offset_r, ray_t).map(|mut rec| {
            rec.p += self.offset;
            rec
        })
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

pub struct Rotation<const T: usize> {
    object: Arc<dyn Hittable>,
    bbox: AABB,
    sin_theta: f64,
    cos_theta: f64,
}

impl Rotation<AXIS_Y> {
    pub fn new(object: impl Hittable + 'static, angle_degrees: f64) -> Self {
        let radians = angle_degrees.to_radians();
        let (sin_theta, cos_theta) = radians.sin_cos();
        let bbox = object.bounding_box();

        let mut min = Point3::INFINITY;
        let mut max = Point3::NEG_INFINITY;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = DVec3::new(new_x, y, new_z);

                    for a in 0..3 {
                        min[a] = min[a].min(tester[a]);
                        max[a] = max[a].max(tester[a]);
                    }
                }
            }
        }

        Self {
            object: Arc::new(object),
            bbox: AABB::from_points(min, max),
            sin_theta,
            cos_theta,
        }
    }
}

impl Hittable for Rotation<AXIS_Y> {
    fn hit<'mat>(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Change ray from world to object space
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::new_with_time(origin, direction, r.time);

        // Determine where an intersection occurs in object space
        if let Some(mut hit) = self.object.hit(&rotated_r, ray_t) {
            // Change the intersection point to world space
            let mut p = hit.p;
            p.x = self.cos_theta * hit.p.x + self.sin_theta * hit.p.z;
            p.z = -self.sin_theta * hit.p.x + self.cos_theta * hit.p.z;
            hit.p = p;

            // Change the normal to world space
            let mut normal = hit.normal;
            normal.x = self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z;
            normal.z = -self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z;
            hit.normal = normal;

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
