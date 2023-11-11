pub mod all;
pub mod color;
pub mod interval;
pub mod perlin;
pub mod random;
pub mod ray;
pub mod vec;

pub type Point3 = glam::Vec3A;

pub fn hermitian_smoothing(d: f32) -> f32 {
    d * d * (3.0 - 2.0 * d)
}
