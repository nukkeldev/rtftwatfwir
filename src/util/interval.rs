use std::{f32::INFINITY, ops::Add};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const UNIVERSE: Self = Interval::new(-INFINITY, INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Self, b: Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn contains(&self, n: f32) -> bool {
        self.min <= n && self.max >= n
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }
}

impl Add<f32> for Interval {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}
