#[derive(Debug, Clone, Copy)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T>
where
    T: PartialEq + PartialOrd,
{
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, n: T) -> bool {
        self.min <= n && self.max >= n
    }
}
