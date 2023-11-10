use crate::{interval::Interval, ray::Ray, Point3};

#[derive(Debug, Default, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new_empty() -> Self {
        Self::default()
    }

    pub fn new(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Self {
            x: ix,
            y: iy,
            z: iz,
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn from_boxes(box_1: &Self, box_2: &Self) -> Self {
        Self {
            x: Interval::from_intervals(box_1.x, box_2.x),
            y: Interval::from_intervals(box_1.y, box_2.y),
            z: Interval::from_intervals(box_1.z, box_2.z),
        }
    }

    pub fn expand(&self, other_box: &Self) -> Self {
        Self::from_boxes(self, other_box)
    }

    pub fn axis(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Axis out of range!"),
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<Interval> {
        for a in 0..2 {
            let inv_d = 1.0 / r.direction[a];
            let orig = r.origin[a];

            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min { ray_t.min = t0; }
            if t1 < ray_t.max { ray_t.max = t1; }

            if ray_t.max <= ray_t.min {
                return None;
            }
        }
        Some(ray_t)
    }
}
