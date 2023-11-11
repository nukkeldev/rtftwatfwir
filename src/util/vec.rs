use glam::Vec3A;

pub fn vec3_a_near_zero(vec: &Vec3A) -> bool {
    vec.x.abs() < 1.0e-8 && vec.y.abs() < 1.0e-8 && vec.z.abs() < 1.0e-8
}

pub fn reflect(v: Vec3A, n: Vec3A) -> Vec3A {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: Vec3A, n: Vec3A, etai_over_etat: f32) -> Vec3A {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -n;
    r_out_perp + r_out_parallel
}

pub const AXIS_X: usize = 0;
pub const AXIS_Y: usize = 1;
pub const AXIS_Z: usize = 2;
