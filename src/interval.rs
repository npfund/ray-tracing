pub struct Interval(pub f64, pub f64);

impl Interval {
    pub fn empty() -> Self {
        Interval(f64::MAX, f64::MIN)
    }

    pub fn universe() -> Self {
        Interval(f64::MIN, f64::MAX)
    }

    pub fn size(&self) -> f64 {
        self.1 - self.0
    }

    pub fn contains(&self, x: f64) -> bool {
        self.0 <= x && x <= self.1
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }
}
