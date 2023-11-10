use glam::DVec3;

use super::{
    hermitian_smoothing,
    random::{random_int_in_range, random_unit_vector},
    Point3,
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ran_vec: [DVec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ran_vec = [DVec3::ZERO; POINT_COUNT];
        for i in 0..POINT_COUNT {
            ran_vec[i] = random_unit_vector();
        }

        Self {
            ran_vec,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[DVec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn terb(&self, p: Point3) -> f64 {
        self._terb(p, 7)
    }

    fn _terb(&self, p: Point3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        for i in (1..POINT_COUNT).rev() {
            p.swap(i, random_int_in_range(0, i as i32) as usize);
        }

        p
    }

    #[allow(unused)]
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        acc
    }

    fn perlin_interp(c: [[[DVec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = hermitian_smoothing(u);
        let vv = hermitian_smoothing(v);
        let ww = hermitian_smoothing(w);

        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = DVec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }
        acc
    }
}
