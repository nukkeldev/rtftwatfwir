use glam::DVec3;

use super::Point3;

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: DVec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, direction: DVec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
