use glam::Vec3A;

use super::Point3;

#[derive(Default, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3A,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3A) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3A, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
