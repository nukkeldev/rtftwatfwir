use glam::Vec3A;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

pub fn random_f32s<const N: usize>(min: f32, max: f32) -> [f32; N] {
    let uniform = Uniform::from(min..max);
    let mut rng = thread_rng();

    let mut out = [0.0; N];

    for i in 0..N {
        out[i] = uniform.sample(&mut rng);
    }

    out
}

pub fn random_f32(min: f32, max: f32) -> f32 {
    random_f32s::<1>(min, max)[0]
}

pub fn two_random_f32s(min: f32, max: f32) -> (f32, f32) {
    let n = random_f32s::<2>(min, max);
    (n[0], n[1])
}

pub fn random_vec_in_range(min: f32, max: f32) -> Vec3A {
    Vec3A::from(random_f32s::<3>(min, max))
}

pub fn random_int_in_range(min: i32, max: i32) -> i32 {
    random_f32s::<1>(min as f32, max as f32)[0].round() as i32
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p = Vec3A::from(random_f32s::<3>(-1.0, 1.0));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3A {
    loop {
        let ns = random_f32s::<2>(-1.0, 1.0);
        let p = Vec3A::new(ns[0], ns[1], 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3A {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: Vec3A) -> Vec3A {
    let on_unit_sphere = random_unit_vector();
    // In the same hemisphere as the normal
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
