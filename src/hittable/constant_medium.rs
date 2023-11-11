use std::{f32::INFINITY, sync::Arc};

use glam::Vec3A;

use crate::{
    bvh::aabb::AABB,
    material::{Isotropic, Material},
    texture::Texture,
};

use super::{HitRecord, Hittable};
use crate::util::all::*;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(b: impl Hittable + 'static, d: f32, a: impl Texture + 'static) -> Self {
        Self {
            boundary: Arc::new(b),
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::new(a),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit<'mat>(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Occasional debugging.
        const ENABLE_DEBUGGING: bool = false;
        let debugging: bool = ENABLE_DEBUGGING && random_f32(0.0, 1.0) < 0.00001;

        if let Some(mut hit1) = self.boundary.hit(r, Interval::UNIVERSE) {
            if let Some(mut hit2) = self
                .boundary
                .hit(r, Interval::new(hit1.t + 0.0001, INFINITY))
            {
                if debugging {
                    eprintln!("ray_tmin={} ray_tmax={}", hit1.t, hit2.t);
                }

                hit1.t = hit1.t.max(ray_t.min);
                hit2.t = hit2.t.min(ray_t.max);

                if hit1.t >= hit2.t {
                    return None;
                }

                hit1.t = hit1.t.max(0.0);

                let ray_length = r.direction.length();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_f32(0.0, 1.0).log10();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;
                let p = r.at(t);

                if debugging {
                    eprintln!("hit_distance={hit_distance}\nt={t}\np={p}");
                }

                let mut rec = HitRecord::new(
                    r,
                    t,
                    p,
                    0.0,
                    0.0,
                    &self.phase_function,
                    Vec3A::X, // Arbitrary
                );

                rec.front_face = true; // Arbitrary

                return Some(rec);
            }
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
