use glam::DVec3;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod aabb;
pub mod bvh_node;

pub type Point3 = glam::DVec3;
pub type Color = glam::DVec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (256.0 * linear_to_gamma(pixel_color.x * scale).clamp(0.0, 0.999)) as i32;
    let g = (256.0 * linear_to_gamma(pixel_color.y * scale).clamp(0.0, 0.999)) as i32;
    let b = (256.0 * linear_to_gamma(pixel_color.z * scale).clamp(0.0, 0.999)) as i32;

    write!(out, "{r} {g} {b}\n").expect("Failed to write color!");
}

pub fn random_ranged_f64s<const N: usize>(min: f64, max: f64) -> [f64; N] {
    let uniform = Uniform::from(min..max);
    let mut rng = thread_rng();

    let mut out = [0.0; N];

    for i in 0..N {
        out[i] = uniform.sample(&mut rng);
    }

    out
}

pub fn random_range(min: f64, max: f64) -> Color {
    DVec3::from(random_ranged_f64s::<3>(min, max))
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_ranged_f64s::<1>(min as f64, max as f64)[0].round() as i32
}

pub fn random_in_unit_sphere() -> DVec3 {
    loop {
        let p = DVec3::from(random_ranged_f64s::<3>(-1.0, 1.0));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> DVec3 {
    loop {
        let ns = random_ranged_f64s::<2>(-1.0, 1.0);
        let p = DVec3::new(ns[0], ns[1], 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector();
    // In the same hemisphere as the normal
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

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