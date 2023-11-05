use glam::DVec3;

use crate::Point3;

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
