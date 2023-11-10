pub mod all;
pub mod color;
pub mod image;
pub mod interval;
pub mod perlin;
pub mod random;
pub mod ray;
pub mod vec;

pub type Point3 = glam::DVec3;

pub fn hermitian_smoothing(d: f64) -> f64 {
    d * d * (3.0 - 2.0 * d)
}
