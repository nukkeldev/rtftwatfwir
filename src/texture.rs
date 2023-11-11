use std::sync::Arc;

use load_image::Image;

use crate::util::{color::Color, image::ImageT, perlin::Perlin, Point3};

pub trait Texture: Send + Sync {
    fn sample(&self, u: f64, v: f64, point: Point3) -> Color;
}

impl Texture for Color {
    fn sample(&self, _u: f64, _v: f64, _point: Point3) -> Color {
        *self
    }
}

/// A Spatial Texture; Does not map to non-cartesian texture coordinate spaces.
pub struct CheckerTexture {
    /// 1.0 / scale
    pub inv_scale: f64,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: impl Texture + 'static, odd: impl Texture + 'static) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(even),
            odd: Arc::new(odd),
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, u: f64, v: f64, point: Point3) -> Color {
        let x_int = (self.inv_scale * point.x).floor() as i32;
        let y_int = (self.inv_scale * point.y).floor() as i32;
        let z_int = (self.inv_scale * point.z).floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.sample(u, v, point)
        } else {
            self.odd.sample(u, v, point)
        }
    }
}

impl Texture for Image {
    fn sample(&self, u: f64, v: f64, _point: Point3) -> Color {
        // DEBUGGING
        if self.height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        };

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.width as f64) as usize;
        let j = (v * self.height as f64) as usize;

        self.get_pixel_data(i, j)
    }
}

pub struct NoiseTexture(Perlin, f64);

impl NoiseTexture {
    pub fn new() -> Self {
        Self(Perlin::new(), 1.0)
    }

    pub fn scaled(scale: f64) -> Self {
        Self(Perlin::new(), scale)
    }
}

impl Texture for NoiseTexture {
    fn sample(&self, _u: f64, _v: f64, point: Point3) -> Color {
        let s = self.1 * point;
        Color::ONE * 0.5 * (1.0 + (s.z + s.x + s.y + 10.0 * self.0.turb(s, 7)).sin())
    }
}
