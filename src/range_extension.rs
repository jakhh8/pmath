use std::ops::RangeInclusive;

pub trait RangeExtension<T> {
    #[allow(unused)]
    fn surrounds(&self, value: T) -> bool;

    #[allow(unused)]
    fn clamp(&self, value: T) -> T;

    #[allow(unused)]
    fn expand(&self, delta: T) -> Self;

    #[allow(unused)]
    fn size(&self) -> T;
}

impl RangeExtension<f64> for RangeInclusive<f64> {
    fn surrounds(&self, value: f64) -> bool {
        *self.start() < value && value < *self.end()
    }

    fn clamp(&self, value: f64) -> f64 {
        value.max(*self.start()).min(*self.end())
    }

    fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;

        self.start() - padding..=self.end() + padding
    }

    fn size(&self) -> f64 {
        self.end() - self.start()
    }
}
