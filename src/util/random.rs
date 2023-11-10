use glam::DVec3;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

pub fn random_f64s<const N: usize>(min: f64, max: f64) -> [f64; N] {
    let uniform = Uniform::from(min..max);
    let mut rng = thread_rng();

    let mut out = [0.0; N];

    for i in 0..N {
        out[i] = uniform.sample(&mut rng);
    }

    out
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    random_f64s::<1>(min, max)[0]
}

pub fn two_random_f64s(min: f64, max: f64) -> (f64, f64) {
    let n = random_f64s::<2>(min, max);
    (n[0], n[1])
}

pub fn random_vec_in_range(min: f64, max: f64) -> DVec3 {
    DVec3::from(random_f64s::<3>(min, max))
}

pub fn random_int_in_range(min: i32, max: i32) -> i32 {
    random_f64s::<1>(min as f64, max as f64)[0].round() as i32
}

pub fn random_in_unit_sphere() -> DVec3 {
    loop {
        let p = DVec3::from(random_f64s::<3>(-1.0, 1.0));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> DVec3 {
    loop {
        let ns = random_f64s::<2>(-1.0, 1.0);
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
