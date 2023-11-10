use glam::DVec3;

pub fn dvec3_near_zero(vec: &DVec3) -> bool {
    vec.x.abs() < 1.0e-8 && vec.y.abs() < 1.0e-8 && vec.z.abs() < 1.0e-8
}

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -n;
    r_out_perp + r_out_parallel
}
