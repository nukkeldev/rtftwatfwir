use glam::Vec3A;
use rand::{seq::SliceRandom, thread_rng};

use super::{hermitian_smoothing, random::random_vec_in_range, Point3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vec3A; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = [Vec3A::ZERO; POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranvec[i] = random_vec_in_range(-1.0, 1.0).normalize();
        }

        Self {
            ranvec,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3A::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ix = self.perm_x[((i + di as i32) & 255) as usize];
                    let iy = self.perm_y[((j + dj as i32) & 255) as usize];
                    let iz = self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.ranvec[(ix ^ iy ^ iz) as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, mut p: Point3, depth: usize) -> f32 {
        let mut acc = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        p.shuffle(&mut thread_rng());

        p
    }

    #[allow(unused)]
    fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f32 * u + (1.0 - i as f32) * (1.0 - u))
                        * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                        * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        acc
    }

    fn perlin_interp(c: [[[Vec3A; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = hermitian_smoothing(u);
        let vv = hermitian_smoothing(v);
        let ww = hermitian_smoothing(w);

        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3A::new(u - i as f32, v - j as f32, w - k as f32);
                    acc += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }
        acc
    }
}
